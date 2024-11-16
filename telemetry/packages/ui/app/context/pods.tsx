import { getTopic } from '@/lib/utils';
import { MQTT_CONNECTION_STATUS } from '@/types/MQTTConnectionStatus';
import {
	POD_CONNECTION_STATUS,
	type PodConnectionStatusType,
} from '@/types/PodConnectionStatus';
import {
	ALL_POD_STATES,
	type ModeType,
	POD_IDS,
	type PodId,
	type PodStateType,
	pods,
} from '@hyped/telemetry-constants';
import { FAILURE_STATES } from '@hyped/telemetry-constants';
import { http } from 'openmct/core/http';
import { createContext, useContext, useEffect, useState } from 'react';
import { ERROR_IDS, useErrors } from './errors';
import { useMQTT } from './mqtt';

/**
 * The maximum latency before a pod is considered disconnected, in milliseconds
 */
const POD_MAX_LATENCY = 300 as const;

/**
 * The latency at which a warning is sent to the server, in milliseconds
 */
const POD_WARNING_LATENCY = 100 as const;

/**
 * The number of previous latencies to keep
 */
const NUM_PREVIOUS_LATENCIES = 50 as const;

/**
 * The number of latencies to use to calculate average
 */
const NUM_LATENCIES_AVG = 10 as const;

/**
 * The interval between latency messages, in milliseconds
 */
const LATENCY_REQUEST_INTERVAL = 100 as const;

/**
 * The default pod ID to use
 */
export const DEFAULT_POD_ID = POD_IDS[1];

export type PreviousLatenciesType = {
	index: number;
	latency: number;
}[];

type PodsStateType = {
	[podId: string]: {
		id: PodId;
		name: string;
		operationMode: ModeType;
		connectionStatus: PodConnectionStatusType;
		previousLatencies?: PreviousLatenciesType;
		latency?: number;
		connectionEstablished?: Date;
		podState: PodStateType;
	};
};

type PodsContextType = {
	pods: PodsStateType;
	currentPod: PodId;
	setCurrentPod: (podId: PodId) => void;
};

const PodsContext = createContext<PodsContextType | null>(null);

/**
 * Creates a pods state object from an array of pod IDs.
 * @param podIds The array of pod IDs to create the pods state object from.
 * @returns The pods state object.
 */
function createPodsStateFromIds(podIds: typeof POD_IDS): PodsStateType {
	const podsContext: PodsStateType = {};
	for (const podId of podIds) {
		podsContext[podId] = {
			id: podId,
			name: pods[podId].name,
			operationMode: pods[podId].operationMode,
			connectionStatus: POD_CONNECTION_STATUS.CONNECTED,
			podState: ALL_POD_STATES.UNKNOWN,
		};
	}
	return podsContext;
}

/**
 * MQTT Context Provider.
 * Provides an MQTT client and functions to publish, subscribe and unsubscribe to MQTT topics.
 * Also provides the connection status of the MQTT client.
 */
export const PodsProvider = ({ children }: { children: React.ReactNode }) => {
	const [podsState, setPodsState] = useState<PodsStateType>(
		createPodsStateFromIds(POD_IDS),
	);
	const [currentPod, setCurrentPod] = useState<PodId>(DEFAULT_POD_ID);
	const [lastLatencyResponse, setLastLatencyResponse] = useState<number>();

	const { client, publish, subscribe, unsubscribe, mqttConnectionStatus } =
		useMQTT();

	const { raiseError } = useErrors();

	useEffect(
		/**
		 * When the MQTT connection status changes, check if we need to set the pod connection statuses to disconnected.
		 */
		function checkMqttConnectionStatus() {
			// If the MQTT connection status has changed to disconnected, set all pod connection statuses to disconnected
			if (mqttConnectionStatus !== MQTT_CONNECTION_STATUS.CONNECTED) {
				setPodsState((prevState) => {
					const newPodsState = { ...prevState };
					for (const podId of POD_IDS) {
						if (
							newPodsState[podId].connectionStatus !==
							POD_CONNECTION_STATUS.DISCONNECTED
						) {
							newPodsState[podId].connectionStatus =
								POD_CONNECTION_STATUS.DISCONNECTED;
							raiseError(
								ERROR_IDS.POD_DISCONNECT,
								`Pod ${podId} disconnected!`,
								`Lost connection to ${podId} because the connection to the MQTT broker has been lost.`,
								podId,
							);
						}
					}
					return newPodsState;
				});
			}
		},
		[mqttConnectionStatus, raiseError],
	);

	// biome-ignore lint/correctness/useExhaustiveDependencies:
	useEffect(
		/**
		 * When the client changes, set up the latency messages.
		 */
		function sendLatencyMessages() {
			// send latency messages every LATENCY_INTERVAL milliseconds
			const interval = setInterval(() => {
				POD_IDS.map((podId) => {
					publish(
						'latency/request',
						JSON.stringify({
							latency: new Date().getTime().toString(),
						}),
						podId,
					);
				});
			}, LATENCY_REQUEST_INTERVAL);
			return () => clearInterval(interval);
		},
		[client],
	);

	useEffect(
		/**
		 * When the last latency response changes, add an interval to check if the pod is disconnected after the `POD_MAX_LATENCY` time.
		 */
		function checkLatency() {
			const interval = setTimeout(() => {
				POD_IDS.map((podId) => {
					if (!lastLatencyResponse) return;
					if (new Date().getTime() - lastLatencyResponse > POD_MAX_LATENCY) {
						setPodsState((prevState) => ({
							...prevState,
							[podId]: {
								...prevState[podId],
								connectionStatus: POD_CONNECTION_STATUS.DISCONNECTED,
								// reset previous latencies
								previousLatencies: [],
								// reset latency
								latency: undefined,
								// reset connection established
								connectionEstablished: undefined,
							},
						}));
						raiseError(
							ERROR_IDS.POD_DISCONNECT,
							`Pod ${podId} disconnected!`,
							`Lost connection to ${podId} because the latency between the base station and the pod is too high.`,
							podId,
						);
					}
				});
			}, POD_MAX_LATENCY);
			return () => clearInterval(interval);
		},
		[lastLatencyResponse, raiseError],
	);

	// biome-ignore lint/correctness/useExhaustiveDependencies:
	useEffect(
		/**
		 * Subscribe to MQTT messages and calculate latencies.
		 */
		function subscribeToMqttAndLatency() {
			if (!client) return;
			const processMessage = (podId: PodId, topic: string, message: Buffer) => {
				if (topic === getTopic('state', podId)) {
					const newPodState = message.toString();
					const allowedStates = Object.values(ALL_POD_STATES);
					if ((allowedStates as string[]).includes(newPodState)) {
						setPodsState((prevState) => ({
							...prevState,
							[podId]: {
								...prevState[podId],
								podState: newPodState as PodStateType,
							},
						}));
					}
					// raise an error if we are in a failure state
					if (Object.keys(FAILURE_STATES).includes(newPodState)) {
						raiseError(
							ERROR_IDS.POD_FAILURE_STATE,
							`Pod ${podId} in failure state!`,
							`Pod ${podId} is in a failure state.`,
							podId,
						);
					}
				} else if (topic === getTopic('latency/response', podId)) {
					// calculate the latency
					const latency =
						new Date().getTime() -
						Number.parseInt(JSON.parse(message.toString()).latency as string);

					// send warning to the server if the latency is too high
					if (latency > POD_WARNING_LATENCY) {
						void http.post(`pods/${podId}/warnings/latency`);
					}

					setLastLatencyResponse(new Date().getTime());

					// update the connection status
					setPodsState((prevState) => ({
						...prevState,
						[podId]: {
							...prevState[podId],
							connectionStatus: POD_CONNECTION_STATUS.CONNECTED,
							// maintain a list of the previous NUM_PREVIOUS_LATENCIES latencies
							previousLatencies: [
								...(prevState[podId].previousLatencies || []).slice(
									-NUM_PREVIOUS_LATENCIES,
								),
								{
									index: new Date().getTime(),
									latency,
								},
							],
							// calculate the average latency from the last NUM_LATENCIES_AVG latencies
							latency: Math.round(
								(prevState[podId].previousLatencies || [])
									.slice(-NUM_LATENCIES_AVG)
									.reduce((acc, { latency }) => acc + latency, 0) /
									NUM_LATENCIES_AVG,
							),
							// set the connection established time if it hasn't been set yet
							connectionEstablished:
								prevState[podId].connectionEstablished || new Date(),
						},
					}));
				}
			};

			// subscribe to latency messages and add MQTT message callback for each pod
			POD_IDS.map((podId) => {
				subscribe('latency/response', podId);
				subscribe('state', podId);
				client.on('message', (topic, message) =>
					processMessage(podId, topic, message),
				);
			});

			return () => {
				POD_IDS.map((podId) => {
					client.off('message', (topic, message) =>
						processMessage(podId, topic as string, message as Buffer),
					);
					unsubscribe('latency/response', podId);
					unsubscribe('state', podId);
				});
			};
		},
		[client],
	);

	const value = {
		pods: podsState,
		currentPod,
		setCurrentPod,
	};

	return <PodsContext.Provider value={value}>{children}</PodsContext.Provider>;
};

/**
 * Hook to get the pod info from the context for the given podId.
 * @param podId The pod ID
 * @returns The pod info
 */
export const usePod = (podId: string) => {
	const context = useContext(PodsContext);
	if (!context) {
		throw new Error('usePods must be used within PodsProvider');
	}
	// Get the pod connection status for the given podId
	return context.pods[podId];
};

/**
 * Hook to use the pods context.
 * @returns The pods context
 */
export const usePods = () => {
	const context = useContext(PodsContext);
	if (!context) {
		throw new Error('usePods must be used within PodsProvider');
	}
	return {
		pods: context.pods,
	};
};

/**
 * Hook to retrieve selected pod info
 * @returns Summary info for the current pod
 */
export const useCurrentPod = () => {
	const context = useContext(PodsContext);
	if (!context) {
		throw new Error('useCurrentPod must be used within PodsProvider');
	}
	return {
		currentPod: context.currentPod,
		pod: context.pods[context.currentPod],
		setCurrentPod: context.setCurrentPod,
	};
};

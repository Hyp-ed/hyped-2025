import { log } from '@/lib/logger';
import { getTopic } from '@/lib/utils';
import {
	type MQTTConnectionStatusType,
	MQTT_CONNECTION_STATUS,
} from '@/types/MQTTConnectionStatus';
import type { MqttPublish, MqttSubscribe, QoS } from '@/types/mqtt';
import type { MqttUnsubscribe } from '@/types/mqtt';
import mqtt from 'mqtt/dist/mqtt';
import type { IClientOptions, MqttClient } from 'mqtt/types/lib/client';
import type { IClientPublishOptions } from 'mqtt/types/lib/client-options';
import { createContext, useContext, useEffect, useState } from 'react';

type MQTTContextType = {
	client: MqttClient | null;
	publish: MqttPublish;
	subscribe: MqttSubscribe;
	unsubscribe: MqttUnsubscribe;
	customPublish: (
		topic: string,
		message: string | Buffer,
		opts: IClientPublishOptions,
	) => MqttClient | undefined;
	mqttConnectionStatus: MQTTConnectionStatusType;
	connectedAt: number | null;
	broker: string;
};

const MQTTContext = createContext<MQTTContextType | null>(null);

interface MQTTProviderProps {
	broker: string;
	qos: QoS;
	children: React.ReactNode;
}

/**
 * MQTT Context Provider.
 * Provides an MQTT client and functions to publish, subscribe and unsubscribe to MQTT topics.
 * Also provides the connection status of the MQTT client.
 */
export const MQTTProvider = ({ broker, qos, children }: MQTTProviderProps) => {
	const [client, setClient] = useState<MqttClient | null>(null);

	const [connectionStatus, setConnectionStatus] =
		useState<MQTTConnectionStatusType>(MQTT_CONNECTION_STATUS.UNKNOWN);

	const [connectedAt, setConnectedAt] = useState<number | null>(null);

	/**
	 * Connect to an MQTT broker
	 * @param broker The broker to connect to
	 * @param mqttOption The MQTT options
	 */
	const mqttConnect = (broker: string, mqttOption?: IClientOptions) => {
		log(`Connecting to MQTT broker: ${broker}`);
		setConnectionStatus(MQTT_CONNECTION_STATUS.CONNECTING);
		const mqttClient = mqtt.connect(broker, mqttOption) as MqttClient;
		setClient(mqttClient);
		setConnectedAt(Date.now());
	};

	// Connect to MQTT broker on mount
	useEffect(
		function connectToMqttBrokerOnMount() {
			mqttConnect(broker);
		},
		[broker],
	);

	// Handle client changes
	useEffect(
		function handleClientChanges() {
			if (client) {
				client.on('connect', () => {
					log('MQTT client connected to broker');
					setConnectionStatus(MQTT_CONNECTION_STATUS.CONNECTED);
				});
				client.on('error', (err: any) => {
					log(`MQTT connection error: ${JSON.stringify(err)}`);
					setConnectionStatus(MQTT_CONNECTION_STATUS.ERROR);
					client.end();
				});
				client.on('reconnect', () => {
					setConnectionStatus(MQTT_CONNECTION_STATUS.RECONNECTING);
				});
			} else {
				log("MQTT client doesn't exist, reconnecting...");
				mqttConnect(broker);
			}
		},
		[client, broker],
	);

	/**
	 * Publish an MQTT message
	 * @param topic The topic to publish to (will be prefixed with ```hyped/{podId}/```)
	 * @param payload The payload to publish
	 * @param podId The pod ID
	 */
	const publish = (topic: string, payload: string, podId: string) => {
		const fullTopic = getTopic(topic, podId);
		if (!client) {
			log(`MQTT couldn't publish to ${fullTopic} because client is null`);
			return;
		}
		client.publish(fullTopic, payload, { qos }, (error: unknown) => {
			if (error) {
				log(`MQTT publish error: ${error as string}`);
			}
		});
	};

	/**
	 * Subscribe to an MQTT topic
	 * @param topic The topic to subscribe to (will be prefixed with ```hyped/{podId}/```)
	 * @param podId The pod ID
	 */
	const subscribe = (topic: string, podId: string) => {
		const fullTopic = getTopic(topic, podId);
		if (!client) {
			log(`MQTT couldn't subscribe to ${fullTopic} because client is null`);
			return;
		}
		client.subscribe(fullTopic, { qos });
	};

	/**
	 * Unsubscribe from an MQTT topic
	 * @param topic The topic to unsubscribe from (will be prefixed with ```hyped/{podId}/```)
	 * @param podId The pod ID
	 * @returns
	 */
	const unsubscribe = (topic: string, podId: string) => {
		const fullTopic = getTopic(topic, podId);
		if (!client) {
			log(`MQTT couldn't unsubscribe from ${fullTopic} because client is null`);
			return;
		}
		client.unsubscribe(fullTopic);
	};

	const customPublish = (
		topic: string,
		message: string | Buffer,
		opts: IClientPublishOptions,
	) => {
		if (!client) {
			log(`MQTT couldn't publish to ${topic} because client is null`);
			return;
		}
		return client.publish(topic, message, opts, (error) => {
			if (error) {
				log(`MQTT publish error: ${error.name} - ${error.message}`);
			}
		});
	};

	return (
		<MQTTContext.Provider
			value={{
				client,
				publish,
				subscribe,
				unsubscribe,
				customPublish,
				mqttConnectionStatus: connectionStatus,
				connectedAt,
				broker,
			}}
		>
			{children}
		</MQTTContext.Provider>
	);
};

export const useMQTT = () => {
	const context = useContext(MQTTContext);
	if (!context) {
		throw new Error('useMQTT must be used within MQTTProvider');
	}
	return context;
};

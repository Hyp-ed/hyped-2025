import { socket as socketConstants } from '@hyped/telemetry-constants';
import type { OpenMCT } from 'openmct/dist/openmct';
import type { MeasurementReading } from 'openmct/types/MeasurementReading';
import type { RealtimeTelemetryListenerCallback } from 'openmct/types/RealtimeTelemetryListenerCallback';
import { io } from 'socket.io-client';
import { SERVER_ENDPOINT } from '../core/config';
import type { AugmentedDomainObject } from '../types/AugmentedDomainObject';

/**
 * The Realtime Telemetry plugin for Open MCT.
 * Provides realtime telemetry data by subscribing to the telemetry server using a websocket.
 * @see https://github.com/nasa/openmct/blob/master/API.md#telemetry-api
 * @returns The realtime telemetry plugin function.
 */
export function RealtimeTelemetryPlugin() {
	return function install(openmct: OpenMCT) {
		const socket = io(SERVER_ENDPOINT, { path: '/openmct/data/realtime' });

		const listenerCallbacks: Record<string, RealtimeTelemetryListenerCallback> =
			{};

		socket.on(socketConstants.MEASUREMENT_EVENT, (data: MeasurementReading) => {
			const { podId, measurementKey, value, timestamp } = data;
			const roomName = socketConstants.getMeasurementRoomName(
				podId,
				measurementKey,
			);

			// When we get a new measurement reading, call the callback for that measurement
			if (listenerCallbacks[roomName]) {
				listenerCallbacks[roomName]({ id: measurementKey, value, timestamp });
			}
		});

		const provider = {
			// We only support realtime telemetry for domain objects that have a podId
			supportsSubscribe: (domainObject: AugmentedDomainObject) => {
				return domainObject.podId !== undefined;
			},
			/**
			 * Subscribes to realtime telemetry data for a domain object.
			 * Sends a subscription request to the server and saves the callback to `listenerCallbacks`.
			 * @param domainObject The domain object to subscribe to.
			 * @param callback The callback to call when new telemetry data is received.
			 * @returns A function to unsubscribe from the telemetry data.
			 */
			subscribe: (
				domainObject: AugmentedDomainObject,
				callback: () => void,
			) => {
				const { podId, identifier } = domainObject;
				const roomName = socketConstants.getMeasurementRoomName(
					podId,
					identifier.key,
				);

				// Save the callback to call when new telemetry data is received
				listenerCallbacks[roomName] = callback;
				socket.emit(socketConstants.EVENTS.SUBSCRIBE_TO_MEASUREMENT, roomName);

				return function unsubscribe() {
					delete listenerCallbacks[roomName];
					socket.emit(
						socketConstants.EVENTS.UNSUBSCRIBE_FROM_MEASUREMENT,
						roomName,
					);
				};
			},
		};

		openmct.telemetry.addProvider(provider);
	};
}

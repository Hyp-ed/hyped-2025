import { socket as socketConstants } from '@hyped/telemetry-constants';
import type { OpenMctFault } from '@hyped/telemetry-types';
import { SERVER_ENDPOINT } from 'openmct/core/config';
import type { AugmentedDomainObject } from 'openmct/types/AugmentedDomainObject';
import { io } from 'socket.io-client';
import {
	FAULT_MANAGEMENT_DOMAIN_TYPE,
	FAULT_MANAGEMENT_TYPES,
} from './constants';

/**
 * The Realtime Faults provider for Open MCT.
 * Provides realtime fault data by subscribing to the telemetry server for faults.
 * @returns The realtime faults provider function.
 */
export function RealtimeFaultsProvider() {
	const socket = io(SERVER_ENDPOINT, { path: '/openmct/faults/realtime' });

	// biome-ignore lint/suspicious/noExplicitAny:
	let faultCallback: any = null;

	// When we get a new fault, call the callback
	socket.on(
		socketConstants.FAULT_EVENT,
		({ fault }: { fault: OpenMctFault }) => {
			// Give Influx time to save to database
			setTimeout(() => {
				if (faultCallback) faultCallback(fault);
			}, 100);
		},
	);

	return {
		// The only domain object type we support is fault management
		supportsSubscribe(domainObject: AugmentedDomainObject) {
			return domainObject.type === FAULT_MANAGEMENT_DOMAIN_TYPE;
		},
		/**
		 * Subscribes to realtime fault data for a domain object.
		 * @param _domainObject (not used)
		 * @param callback The callback to call when new fault data is received.
		 * @returns A function to unsubscribe from the fault data.
		 */
		subscribe: (
			_domainObject: AugmentedDomainObject,
			// biome-ignore lint/suspicious/noExplicitAny:
			callback: (args: any) => void,
		) => {
			socket.emit(socketConstants.EVENTS.SUBSCRIBE_TO_FAULTS);

			faultCallback = callback;

			// Call once first to get the current state
			callback({ type: FAULT_MANAGEMENT_TYPES.global });

			return function unsubscribe() {
				faultCallback = null;
				socket.emit(socketConstants.EVENTS.UNSUBSCRIBE_FROM_FAULTS);
			};
		},
	};
}

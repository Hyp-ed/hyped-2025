import type { HistoricalFaults } from '@hyped/telemetry-types/dist/openmct/openmct-fault.types';
import { http } from 'openmct/core/http';
import type { AugmentedDomainObject } from 'openmct/types/AugmentedDomainObject';
import { FAULT_MANAGEMENT_DOMAIN_TYPE } from './constants';

/**
 * The Historical Faults provider for Open MCT.
 * Provides historical fault data by querying the telemetry server for faults.
 * @returns The historical faults provider function.
 */
export function HistoricalFaultsProvider() {
	return {
		// The only domain object type we support is fault management
		supportsRequest(domainObject: AugmentedDomainObject) {
			return domainObject.type === FAULT_MANAGEMENT_DOMAIN_TYPE;
		},
		/**
		 * Fetches historical fault data for a domain object from the telemetry server.
		 * @returns The historical fault data.
		 */
		request: async () => {
			const url = 'openmct/faults/historical';
			const data = await http.get(url).json<HistoricalFaults>();
			// biome-ignore lint/suspicious/noExplicitAny:
			return data.map((fault: any) => fault.fault);
		},
	};
}

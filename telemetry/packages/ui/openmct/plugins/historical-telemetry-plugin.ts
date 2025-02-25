import type { OpenMCT } from 'openmct/dist/openmct';
import type { TelemetryRequestOptions } from 'openmct/dist/src/api/telemetry/TelemetryAPI';
import { http } from '../core/http';
import type { AugmentedDomainObject } from '../types/AugmentedDomainObject';

/**
 * The Historical Telemetry plugin for Open MCT.
 * Provides historical telemetry data by querying the telemetry server for data
 * from a given start and end time.
 * @see https://github.com/nasa/openmct/blob/master/API.md#telemetry-api
 * @returns The historical telemetry plugin function.
 */
export function HistoricalTelemetryPlugin() {
	return function install(openmct: OpenMCT) {
		const provider = {
			// We only support historical telemetry for domain objects that have a podId
			supportsRequest: (domainObject: AugmentedDomainObject) =>
				domainObject.podId !== undefined,
			request: async (
				domainObject: AugmentedDomainObject,
				options: { start: number; end: number } & TelemetryRequestOptions,
			) => {
				const { start, end } = options;
				const podId = domainObject.podId;
				const measurementKey = domainObject.identifier.key;

				// Fetch historical telemetry data from the server
				const url = `openmct/data/historical/pods/${podId}/measurements/${measurementKey}?start=${start}&end=${end}`;
				const data = await http.get(url).json();
				return data;
			},
		};

		openmct.telemetry.addProvider(provider);
	};
}

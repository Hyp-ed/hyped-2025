import { podIds, pods } from '@hyped/telemetry-constants';
import type { OpenMctDictionary, OpenMctPod } from '@hyped/telemetry-types';
import { Injectable } from '@nestjs/common';
import { mapMeasurementToOpenMct } from './utils/map-to-openmct';

@Injectable()
export class DictionaryService {
	getDictionary(): OpenMctDictionary {
		const dictionary: OpenMctDictionary = {};
		for (const podId of podIds) {
			dictionary[podId] = this.getPod(podId);
		}
		return dictionary;
	}

	getPodIds() {
		return podIds;
	}

	getPod(podId: string): OpenMctPod {
		this.validatePodId(podId);
		const pod = pods[podId];

		const measurements = Object.values(pod.measurements).map((measurement) =>
			mapMeasurementToOpenMct(measurement),
		);

		return {
			name: pod.label,
			id: pod.id,
			measurements,
		};
	}

	getMeasurement(podId: string, measurementKey: string) {
		this.validatePodId(podId);
		const pod = pods[podId];

		const measurement = pod.measurements[measurementKey];
		if (!measurement) {
			throw new Error(`Measurement ${measurementKey} not found`);
		}

		return mapMeasurementToOpenMct(measurement);
	}

	private validatePodId(podId: string) {
		if (!podIds.includes(podId)) {
			throw new Error(`Pod ${podId} not found`);
		}
	}
}

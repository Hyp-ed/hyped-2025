import type { FaultLevel } from '@hyped/telemetry-constants';
import type { RangeMeasurement } from '@hyped/telemetry-types/dist/pods/pods.types';
import type { MeasurementReading } from '../MeasurementReading.types';

export type DoesMeasurementBreachLimitsReturn = false | FaultLevel;

export function doesMeasurementBreachLimits(
	measurement: RangeMeasurement,
	reading: MeasurementReading,
): DoesMeasurementBreachLimitsReturn {
	const { value } = reading;

	const { low, high } = measurement.limits.critical;
	if (value < low || value > high) {
		return 'CRITICAL';
	}

	if (measurement.limits.warning) {
		const { low, high } = measurement.limits.warning;

		if (value < low || value > high) {
			return 'WARNING';
		}
	}

	return false;
}

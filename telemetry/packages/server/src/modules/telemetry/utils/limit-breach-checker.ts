import type { FaultLevel } from '@hyped/telemetry-constants';
import type { Measurement } from '@hyped/telemetry-types';
import type { MeasurementReading } from '../MeasurementReading.types';

export type DoesMeasurementBreachLimitsReturn = false | FaultLevel;

export function doesMeasurementBreachLimits(
	measurement: Measurement,
	value: MeasurementReading['value'],
): DoesMeasurementBreachLimitsReturn {
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

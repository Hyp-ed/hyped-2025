import type { Measurement, OpenMctMeasurement } from '@hyped/telemetry-types';

export function mapMeasurementToOpenMct(
	measurement: Measurement,
): OpenMctMeasurement {
	return {
		name: measurement.label,
		key: measurement.id,
		type: measurement.type,
		values: [
			{
				key: 'value',
				name: measurement.label,
				unit: measurement.unit,
				format: measurement.format,
				min: measurement.limits?.critical.low,
				max: measurement.limits?.critical.high,
				limits: measurement.limits,
				hints: {
					range: 1,
				},
			},
			{
				key: 'utc',
				source: 'timestamp',
				name: 'Timestamp',
				format: 'utc',
				hints: {
					domain: 1,
				},
			},
		],
	};
}

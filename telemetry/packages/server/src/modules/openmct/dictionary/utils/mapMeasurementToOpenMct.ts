import type { Measurement, OpenMctMeasurement } from '@hyped/telemetry-types';

export function mapMeasurementToOpenMct(
	measurement: Measurement,
): OpenMctMeasurement {
	return {
		name: measurement.name,
		key: measurement.key,
		type: measurement.type,
		values: [
			{
				key: 'value',
				name: measurement.name,
				unit: measurement.unit,
				format: measurement.format,
				...('limits' in measurement && {
					min: measurement.limits?.critical.low,
					max: measurement.limits?.critical.high,
					limits: measurement.limits,
				}),
				...('enumerations' in measurement && {
					enumerations: measurement.enumerations,
				}),
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

import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import type { RealtimeTelemetryDataGateway } from '@/modules/openmct/data/realtime/RealtimeTelemetryData.gateway';
import type { FaultService } from '@/modules/openmct/faults/Fault.service';
import { pods } from '@hyped/telemetry-constants';
import { Point } from '@influxdata/influxdb-client';
import { Injectable, type LoggerService } from '@nestjs/common';
import {
	type MeasurementReading,
	MeasurementReadingSchema,
} from './MeasurementReading.types';
import { MeasurementReadingValidationError } from './errors/MeasurementReadingValidationError';
import { doesMeasurementBreachLimits } from './utils/limit-breach-checker';

@Injectable()
export class MeasurementService {
	constructor(
		@Logger()
		private readonly logger: LoggerService,
		private influxService: InfluxService,
		private realtimeDataGateway: RealtimeTelemetryDataGateway,
		private faultService: FaultService,
	) {}

	// This function _is_ ordered in importance
	public async addMeasurementReading(props: MeasurementReading) {
		const validatedMeasurement = this.validateMeasurementReading(props);

		if (!validatedMeasurement) {
			throw new MeasurementReadingValidationError('Invalid measurement');
		}

		const { measurement, reading } = validatedMeasurement;
		const { podId, measurementKey, value, timestamp } = reading;

		// First, get the data to the client ASAP
		this.realtimeDataGateway.sendMeasurementReading({
			podId,
			measurementKey,
			value,
			timestamp,
		});

		// Then check if it breaches limits
		const breachLevel = doesMeasurementBreachLimits(measurement, value);

		if (breachLevel) {
			this.logger.debug(
				`Measurement breached limits {${props.podId}/${props.measurementKey}}: ${breachLevel} with value ${props.value}`,
				MeasurementService.name,
			);
			await this.faultService.addLimitBreachFault({
				level: breachLevel,
				measurement,
				tripReading: reading,
			});
		}

		// Then save it to the database
		const point = new Point('measurement')
			.timestamp(timestamp)
			.tag('podId', podId)
			.tag('measurementKey', measurementKey)
			.floatField('value', value);

		try {
			this.influxService.telemetryWrite.writePoint(point);

			this.logger.debug(
				`Added measurement {${props.podId}/${props.measurementKey}}: ${props.value}`,
				MeasurementService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to add measurement {${props.podId}/${props.measurementKey}}: ${props.value}`,
				e,
				MeasurementService.name,
			);
		}
	}

	private validateMeasurementReading(props: MeasurementReading) {
		const result = MeasurementReadingSchema.safeParse(props);

		if (!result.success) {
			throw new MeasurementReadingValidationError(result.error.message);
		}

		const { podId, measurementKey } = result.data;

		const possibleMeasurement = pods?.[podId]?.measurements?.[measurementKey];

		if (!possibleMeasurement) {
			throw new MeasurementReadingValidationError(
				`Measurement ${measurementKey} not found for pod ${podId}`,
			);
		}

		return {
			reading: result.data,
			measurement: possibleMeasurement,
		};
	}
}

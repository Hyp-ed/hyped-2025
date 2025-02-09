import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import type { RealtimeTelemetryDataGateway } from '@/modules/openmct/data/realtime/RealtimeTelemetryData.gateway';
import type { FaultService } from '@/modules/openmct/faults/Fault.service';
import { pods } from '@hyped/telemetry-constants';
import { Point } from '@influxdata/influxdb-client';
import { Injectable, type LoggerService } from '@nestjs/common';
import { type StatusEntry, StatusEntrySchema } from './StatusEntry.types';
import { StatusEntryValidationError } from './errors/StatusEntryValidationError';

@Injectable()
export class StatusService {
	constructor(
		@Logger()
		private readonly logger: LoggerService,
		private influxService: InfluxService,
		private realtimeDataGateway: RealtimeTelemetryDataGateway,
	) {}

	// This function _is_ ordered in importance
	public async addStatusEntry(props: StatusEntry) {
		const validatedStatusEntry = this.validateStatusEntry(props);

		if (!validatedStatusEntry) {
			throw new StatusEntryValidationError('Invalid status entry');
		}

		const {
			status,
			entry: { podId, value, timestamp },
		} = validatedStatusEntry;

		// First, get the data to the client ASAP
		this.realtimeDataGateway.sendMeasurementReading({
			podId,
			measurementKey: status.id, // temp
			value,
			timestamp,
		});

		// Then save it to the database
		const point = new Point('measurement') // temp until we switch to postgres
			.timestamp(timestamp)
			.tag('podId', podId)
			.tag('measurementKey', status.id)
			.floatField('value', value);

		try {
			this.influxService.telemetryWrite.writePoint(point);

			this.logger.debug(
				`Added status {${props.podId}/${props.statusId}}: ${props.value}`,
				StatusService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to add status {${props.podId}/${props.statusId}}: ${props.value}`,
				e,
				StatusService.name,
			);
		}
	}

	private validateStatusEntry(props: StatusEntry) {
		const result = StatusEntrySchema.safeParse(props);

		if (!result.success) {
			throw new StatusEntryValidationError(result.error.message);
		}

		const { podId, statusId, value } = result.data;

		const possibleStatus = pods?.[podId]?.statuses?.[statusId];

		if (!possibleStatus) {
			throw new StatusEntryValidationError(
				`Status ${statusId} not found for pod ${podId}`,
			);
		}

		if (!possibleStatus.values.map((v) => v.value).includes(value)) {
			throw new StatusEntryValidationError(
				`Status '${statusId}' value '${value}' is not valid`,
			);
		}

		return {
			status: possibleStatus,
			entry: result.data,
		};
	}
}

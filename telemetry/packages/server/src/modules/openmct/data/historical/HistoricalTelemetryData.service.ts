import { INFLUX_TELEMETRY_BUCKET } from '@/core/config';
import type { InfluxRow } from '@/modules/common/types/InfluxRow';
import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import { flux, fluxDateTime } from '@influxdata/influxdb-client';
import { HttpException, Injectable, type LoggerService } from '@nestjs/common';

interface InfluxHistoricalRow extends InfluxRow {
	measurementKey: string;
	format: string;
}

@Injectable()
export class HistoricalTelemetryDataService {
	constructor(
		private influxService: InfluxService,
		@Logger()
		private readonly logger: LoggerService,
	) {}

	public async getHistoricalReading(
		podId: string,
		measurementKey: string,
		startTimestamp: string,
		endTimestamp: string,
	) {
		const fluxStart = fluxDateTime(
			new Date(Number.parseInt(startTimestamp)).toISOString(),
		);
		const fluxEnd = fluxDateTime(
			new Date(Number.parseInt(endTimestamp)).toISOString(),
		);

		const query = flux`
      from(bucket: "${INFLUX_TELEMETRY_BUCKET}")
        |> range(start: ${fluxStart}, stop: ${fluxEnd})
        |> filter(fn: (r) => r["measurementKey"] == "${measurementKey}")
        |> filter(fn: (r) => r["podId"] == "${podId}")`;

		try {
			const data =
				await this.influxService.query.collectRows<InfluxHistoricalRow>(query);

			return data.map((row) => ({
				id: row.measurementKey,
				timestamp: new Date(row._time).getTime(),
				value: row._value,
			}));
		} catch (e: unknown) {
			this.logger.error(
				`Failed to get historical reading for {${podId}/${measurementKey}}`,
				e,
				HistoricalTelemetryDataService.name,
			);
			throw new HttpException("Couldn't get historical reading", 500);
		}
	}
}

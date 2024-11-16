import { INFLUX_FAULTS_BUCKET } from '@/core/config';
import type { InfluxRow } from '@/modules/common/types/InfluxRow';
import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import type { OpenMctFault } from '@hyped/telemetry-types';
import type { HistoricalFaults } from '@hyped/telemetry-types/dist/openmct/openmct-fault.types';
import { fluxString } from '@influxdata/influxdb-client';
import { HttpException, Injectable, type LoggerService } from '@nestjs/common';

interface InfluxFaultRow extends InfluxRow {
	faultId: string;
	measurementKey: string;
	/**
	 * This is the result of JSON.stringify on an OpenMctFault
	 */
	openMctFault: string;
}

type GetHistoricalFaultsInput = {
	podId?: string;
	measurementKey?: string;
	faultId?: string;
};

@Injectable()
export class HistoricalFaultDataService {
	constructor(
		private influxService: InfluxService,
		@Logger()
		private readonly logger: LoggerService,
	) {}

	public async getHistoricalFaults(
		props: GetHistoricalFaultsInput,
	): Promise<HistoricalFaults> {
		const { podId, measurementKey, faultId } = props;

		const query = `from(bucket: "${INFLUX_FAULTS_BUCKET}")
      |> range(start: -24h)
      ${podId ? `|> filter(fn: (r) => r["podId"] == ${fluxString(podId) as unknown as string})` : ''}
      ${
				measurementKey
					? `|> filter(fn: (r) => r["measurementKey"] == ${
							fluxString(measurementKey) as unknown as string
						})`
					: ''
			}
      ${
				faultId
					? `|> filter(fn: (r) => r["faultId"] == ${fluxString(faultId) as unknown as string})`
					: ''
			}
      |> group(columns: ["faultId"])
      |> last()`;

		try {
			const data =
				await this.influxService.query.collectRows<InfluxFaultRow>(query);
			return data.map((row) => ({
				faultId: row['faultId'],
				timestamp: new Date(row['_time']).getTime(),
				openMctFault: JSON.parse(row['_value']) as OpenMctFault,
				podId: row['podId'],
				measurementKey: row['measurementKey'],
			}));
		} catch (e: unknown) {
			this.logger.error(
				`Failed to get faults for pod ${podId}`,
				e,
				HistoricalFaultDataService.name,
			);
			throw new HttpException("Couldn't get historical faults", 500);
		}
	}
}

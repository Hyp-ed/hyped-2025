import {
	INFLUX_FAULTS_BUCKET,
	INFLUX_ORG,
	INFLUX_TELEMETRY_BUCKET,
	INFLUX_TOKEN,
	INFLUX_URL,
} from '@/modules/core/config';
import { Logger } from '@/modules/logger/Logger.decorator';
import {
	InfluxDB,
	type QueryApi,
	type WriteApi,
} from '@influxdata/influxdb-client';
import {
	Injectable,
	type LoggerService,
	type OnModuleInit,
} from '@nestjs/common';
import { InfluxServiceError } from './errors/InfluxServiceError';

@Injectable()
export class InfluxService implements OnModuleInit {
	private connection: InfluxDB;
	public telemetryWrite: WriteApi;
	public faultsWrite: WriteApi;
	public query: QueryApi;
	@Logger()
	private readonly logger: LoggerService;

	async $connect() {
		this.connection = new InfluxDB({ url: INFLUX_URL, token: INFLUX_TOKEN });
		this.telemetryWrite = this.connection.getWriteApi(
			INFLUX_ORG,
			INFLUX_TELEMETRY_BUCKET,
			'ns',
			{
				batchSize: 1,
			},
		);
		this.faultsWrite = this.connection.getWriteApi(
			INFLUX_ORG,
			INFLUX_FAULTS_BUCKET,
			'ns',
			{
				batchSize: 1,
			},
		);
		this.query = this.connection.getQueryApi(INFLUX_ORG);

		// Warn if InfluxDB isn't running
		try {
			await this.query.collectRows('from(bucket: "_monitoring")');
		} catch (e) {
			this.logger.warn(
				"InfluxDB doesn't seem to be running",
				InfluxService.name,
			);
		}
	}

	async onModuleInit() {
		await this.$connect();
		if (!this.telemetryWrite) {
			throw new InfluxServiceError(
				'InfluxDB telemetry write API not initialized',
			);
		}

		if (!this.faultsWrite) {
			throw new InfluxServiceError('InfluxDB faults write API not initialized');
		}

		if (!this.query) {
			throw new InfluxServiceError('InfluxDB query API not initialized');
		}
	}

	async onModuleDestroy() {
		try {
			await this.telemetryWrite.close();
			await this.faultsWrite.close();
		} catch (e) {
			throw new InfluxServiceError('Failed to close InfluxDB');
		}
	}
}

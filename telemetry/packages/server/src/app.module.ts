import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { PodControlsModule } from './modules/controls/PodControls.module';
import { InfluxModule } from './modules/influx/Influx.module';
import { LiveLogsGateway } from './modules/live-logs/LiveLogs.gateway';
import { LoggerModule } from './modules/logger/Logger.module';
import { MqttClientModule } from './modules/mqtt/client/MqttClientModule';
import { MqttIngestionModule } from './modules/mqtt/ingestion/MqttIngestion.module';
import { OpenMCTModule } from './modules/openmct/OpenMCT.module';
import { FaultModule } from './modules/openmct/faults/Fault.module';
import { PublicDataModule } from './modules/public-data/PublicData.module';
import { RemoteLogsModule } from './modules/remote-logs/RemoteLogs.module';
import { TelemetryModule } from './modules/telemetry/Telemetry.module';
import { WarningsModule } from './modules/warnings/Warnings.module';

@Module({
	imports: [
		LoggerModule,
		MqttClientModule,
		InfluxModule,
		MqttIngestionModule,
		OpenMCTModule,
		TelemetryModule,
		FaultModule,
		PodControlsModule,
		WarningsModule,
		RemoteLogsModule,
		PublicDataModule,
	],
	controllers: [AppController],
	providers: [AppService, LiveLogsGateway],
})
export class AppModule {}

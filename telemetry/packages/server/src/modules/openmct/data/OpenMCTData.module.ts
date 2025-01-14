import { Module } from '@nestjs/common';
import { InfluxModule } from 'src/modules/influx/Influx.module';
import { HistoricalTelemetryDataController } from './historical/HistoricalTelemetryData.controller';
import { HistoricalTelemetryDataService } from './historical/HistoricalTelemetryData.service';
import { RealtimeTelemetryDataGateway } from './realtime/RealtimeTelemetryData.gateway';

@Module({
	imports: [InfluxModule],
	controllers: [HistoricalTelemetryDataController],
	providers: [HistoricalTelemetryDataService, RealtimeTelemetryDataGateway],
	exports: [RealtimeTelemetryDataGateway],
})
export class OpenMCTDataModule {}

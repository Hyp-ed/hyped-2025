import { InfluxModule } from '@/modules/influx/Influx.module';
import { HistoricalTelemetryDataService } from '@/modules/openmct/data/historical/HistoricalTelemetryData.service';
import { Module } from '@nestjs/common';
import { PublicDataController } from './PublicData.controller';
import { PublicDataService } from './PublicData.service';

@Module({
	imports: [InfluxModule],
	controllers: [PublicDataController],
	providers: [PublicDataService, HistoricalTelemetryDataService],
})
export class PublicDataModule {}

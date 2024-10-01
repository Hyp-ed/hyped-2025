import { Module } from '@nestjs/common';
import { PublicDataController } from './PublicData.controller';
import { PublicDataService } from './PublicData.service';
import { HistoricalTelemetryDataService } from '@/modules/openmct/data/historical/HistoricalTelemetryData.service';
import { InfluxModule } from '@/modules/influx/Influx.module';

@Module({
  imports: [InfluxModule],
  controllers: [PublicDataController],
  providers: [PublicDataService, HistoricalTelemetryDataService],
})
export class PublicDataModule {}

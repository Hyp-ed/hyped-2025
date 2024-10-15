import { InfluxModule } from '@/modules/influx/Influx.module';
import { Module } from '@nestjs/common';
import { FaultService } from './Fault.service';
import { HistoricalFaultDataService } from './data/historical/HistoricalFaultData.service';
import { RealtimeFaultDataGateway } from './data/realtime/RealtimeFaultData.gateway';
import { HistoricalFaultsDataController } from './data/historical/HistoricalFaultData.controller';
import { FaultsController } from './Fault.controller';

@Module({
  imports: [InfluxModule],
  controllers: [FaultsController, HistoricalFaultsDataController],
  providers: [
    FaultService,
    HistoricalFaultDataService,
    RealtimeFaultDataGateway,
  ],
  exports: [FaultService],
})
export class FaultModule {}

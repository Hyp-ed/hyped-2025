import { Controller, Get, Param, Query } from '@nestjs/common';
import { HistoricalTelemetryDataService } from './HistoricalTelemetryData.service';

@Controller('openmct/data/historical')
export class HistoricalTelemetryDataController {
  constructor(private historicalDataService: HistoricalTelemetryDataService) {}

  @Get('pods/:podId/measurements/:measurementKey')
  getHistoricalReading(
    @Param('podId') podId: string,
    @Param('measurementKey') measurementKey: string,
    @Query('start') start: string,
    @Query('end') end: string,
  ) {
    return this.historicalDataService.getHistoricalReading(
      podId,
      measurementKey,
      start,
      end,
    );
  }
}

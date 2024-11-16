import type { OpenMctHistoricalFaults } from '@hyped/telemetry-types/dist/openmct/openmct-fault.types';
import { Controller, Get, Param } from '@nestjs/common';
import type { HistoricalFaultDataService } from './HistoricalFaultData.service';

@Controller('openmct/faults/historical')
export class HistoricalFaultsDataController {
  constructor(private historicalDataService: HistoricalFaultDataService) {}

  @Get()
  async getAllFaults(): Promise<OpenMctHistoricalFaults> {
    const faults = await this.historicalDataService.getHistoricalFaults({});
    return faults.map((fault) => ({
      timestamp: fault.timestamp,
      fault: fault.openMctFault,
    }));
  }

  @Get('pods/:podId')
  async getFaultsForPod(
    @Param('podId') podId: string,
  ): Promise<OpenMctHistoricalFaults> {
    const faults = await this.historicalDataService.getHistoricalFaults({
      podId,
    });
    return faults.map((fault) => ({
      timestamp: fault.timestamp,
      fault: fault.openMctFault,
    }));
  }
}

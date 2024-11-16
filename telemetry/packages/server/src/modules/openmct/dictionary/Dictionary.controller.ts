import type { OpenMctPod } from '@hyped/telemetry-types';
import { Controller, Get, Param } from '@nestjs/common';
import type { DictionaryService } from './Dictionary.service';

@Controller('openmct/dictionary')
export class DictionaryController {
  constructor(private dictionaryService: DictionaryService) {}

  @Get('pods')
  getPodIds() {
    const ids = this.dictionaryService.getPodIds();
    return {
      ids,
    };
  }

  @Get('pods/:podId')
  getPod(@Param('podId') podId: string): OpenMctPod {
    return this.dictionaryService.getPod(podId);
  }

  @Get('pods/:podId/measurements/:measurementKey')
  getMeasurement(
    @Param('podId') podId: string,
    @Param('measurementKey') measurementKey: string,
  ) {
    return this.dictionaryService.getMeasurement(podId, measurementKey);
  }
}

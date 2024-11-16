import { Controller, Param, Post } from '@nestjs/common';
import type { WarningsService } from './Warnings.service';

@Controller('pods/:podId/warnings')
export class WarningsController {
  constructor(private warningsService: WarningsService) {}

  @Post('latency')
  createLatencyWarning(@Param('podId') podId: string) {
    this.warningsService.createLatencyWarning(podId);
  }
}

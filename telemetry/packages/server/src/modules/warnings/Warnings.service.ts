import { Logger } from '@/modules/logger/Logger.decorator';
import { Injectable, type LoggerService } from '@nestjs/common';

@Injectable()
export class WarningsService {
  constructor(
    @Logger()
    private readonly logger: LoggerService,
  ) {}

  createLatencyWarning(podId: string) {
    this.logger.log(
      `Creating latency warning for pod ${podId}`,
      WarningsService.name,
    );
  }
}

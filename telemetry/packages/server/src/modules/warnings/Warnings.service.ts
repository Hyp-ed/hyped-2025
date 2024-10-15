import { Injectable, LoggerService } from '@nestjs/common';
import { Logger } from '@/modules/logger/Logger.decorator';

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

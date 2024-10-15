import { Injectable, LoggerService } from '@nestjs/common';
import { Logger } from '@/modules/logger/Logger.decorator';

@Injectable()
export class RemoteLogsService {
  constructor(
    @Logger()
    private readonly logger: LoggerService,
  ) {}

  /**
   * Logs a message from the GUI, which is not associated with a particular pod.
   * @param message The message from the GUI to log
   * @returns True if the message was logged successfully, false otherwise
   */
  logRemoteMessage(message: string) {
    this.logger.verbose(`[GUI] ${message}`, RemoteLogsService.name);
    return true;
  }

  /**
   * Logs a message from the GUI, which is associated with a particular pod.
   * @param podId The ID of the pod
   * @param message The message from the GUI to log
   * @returns True if the message was logged successfully, false otherwise
   */
  logRemoteMessageWithPodID(podId: string, message: string) {
    this.logger.verbose(
      `[GUI - Pod ${podId}] ${message}`,
      RemoteLogsService.name,
    );
    return true;
  }
}

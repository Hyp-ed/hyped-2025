import { Module } from '@nestjs/common';
import { RemoteLogsController } from './RemoteLogs.controller';
import { RemoteLogsService } from './RemoteLogs.service';

@Module({
  controllers: [RemoteLogsController],
  providers: [RemoteLogsService],
})
export class RemoteLogsModule {}

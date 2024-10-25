import { Module } from '@nestjs/common';
import { PodControlsController } from './PodControls.controller';
import { PodControlsService } from './PodControls.service';

@Module({
  controllers: [PodControlsController],
  providers: [PodControlsService],
})
export class PodControlsModule {}

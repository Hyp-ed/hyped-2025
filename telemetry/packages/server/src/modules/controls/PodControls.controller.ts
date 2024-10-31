import { Controller, Param, Post, Query } from '@nestjs/common';
import { PodControlsService } from './PodControls.service';

@Controller('pods/:podId/controls')
export class PodControlsController {
  constructor(private podControlsService: PodControlsService) {}

  @Post('levitation-height')
  setLevitationHeight(
    @Param('podId') podId: string,
    @Query('height') height: number,
  ) {
    return this.podControlsService.setLevitationHeight(height, podId);
  }

  @Post(':control')
  controlPod(@Param('control') control: string, @Param('podId') podId: string) {
    return this.podControlsService.sendControlMessage(control, podId);
  }
}

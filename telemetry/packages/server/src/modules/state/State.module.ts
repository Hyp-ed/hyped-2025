import { Module } from '@nestjs/common';
import { StateService } from './State.service';
import { InfluxModule } from '../influx/Influx.module';

@Module({
  imports: [InfluxModule],
  providers: [StateService],
  exports: [StateService],
})
export class StateModule {}

import { Module } from '@nestjs/common';
import { InfluxModule } from '../influx/Influx.module';
import { StateService } from './State.service';

@Module({
	imports: [InfluxModule],
	providers: [StateService],
	exports: [StateService],
})
export class StateModule {}

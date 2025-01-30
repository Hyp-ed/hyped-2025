import { StateModule } from '@/modules/state/State.module';
import { Module } from '@nestjs/common';
import { MeasurementModule } from 'src/modules/measurement/Measurement.module';
import { MqttIngestionService } from './MqttIngestion.service';

@Module({
	imports: [MeasurementModule, StateModule],
	providers: [MqttIngestionService],
})
export class MqttIngestionModule {}

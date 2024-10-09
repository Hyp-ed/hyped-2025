import { Module } from '@nestjs/common';
import { MqttIngestionService } from './MqttIngestion.service';
import { MeasurementModule } from 'src/modules/measurement/Measurement.module';
import { StateModule } from '@/modules/state/State.module';

@Module({
  imports: [MeasurementModule, StateModule],
  providers: [MqttIngestionService],
})
export class MqttIngestionModule {}

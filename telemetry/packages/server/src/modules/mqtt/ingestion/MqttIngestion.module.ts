import { StateModule } from '@/modules/state/State.module';
import { TelemetryModule } from '@/modules/telemetry/Telemetry.module';
import { Module } from '@nestjs/common';
import { MqttIngestionService } from './MqttIngestion.service';

@Module({
	imports: [TelemetryModule, StateModule],
	providers: [MqttIngestionService],
})
export class MqttIngestionModule {}

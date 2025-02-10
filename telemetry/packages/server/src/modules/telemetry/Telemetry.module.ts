import { InfluxModule } from '@/modules/influx/Influx.module';
import { OpenMCTDataModule } from '@/modules/openmct/data/OpenMCTData.module';
import { FaultModule } from '@/modules/openmct/faults/Fault.module';
import { Module } from '@nestjs/common';
import { MeasurementService } from './Measurement.service';

@Module({
	imports: [InfluxModule, OpenMCTDataModule, FaultModule],
	providers: [MeasurementService],
	exports: [MeasurementService],
})
export class TelemetryModule {}

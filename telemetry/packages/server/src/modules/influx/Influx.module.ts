import { Module } from '@nestjs/common';
import { InfluxService } from './Influx.service';

@Module({
  providers: [InfluxService],
  exports: [InfluxService],
})
export class InfluxModule {}

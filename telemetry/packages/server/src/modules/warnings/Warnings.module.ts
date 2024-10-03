import { Module } from '@nestjs/common';
import { WarningsController } from './Warnings.controller';
import { WarningsService } from './Warnings.service';

@Module({
  controllers: [WarningsController],
  providers: [WarningsService],
})
export class WarningsModule {}

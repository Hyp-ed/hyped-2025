import { Module } from '@nestjs/common';
import { OpenMCTDataModule } from './data/OpenMCTData.module';
import { DictionaryController } from './dictionary/Dictionary.controller';
import { DictionaryService } from './dictionary/Dictionary.service';
import { ObjectTypesController } from './object-types/ObjectTypes.controller';
import { ObjectTypesService } from './object-types/ObjectTypes.service';

@Module({
	imports: [OpenMCTDataModule],
	controllers: [DictionaryController, ObjectTypesController],
	providers: [DictionaryService, ObjectTypesService],
})
export class OpenMCTModule {}

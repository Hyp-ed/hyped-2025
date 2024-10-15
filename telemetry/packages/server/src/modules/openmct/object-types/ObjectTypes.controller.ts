import { Controller, Get } from '@nestjs/common';
import { ObjectTypesService as ObjectTypesService } from './ObjectTypes.service';

@Controller('openmct/object-types')
export class ObjectTypesController {
  constructor(private objectTypesService: ObjectTypesService) {}

  @Get()
  getObjectTypes() {
    return this.objectTypesService.getObjectTypes();
  }
}

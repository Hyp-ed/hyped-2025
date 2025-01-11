import { Body, Controller, Post } from '@nestjs/common';
import type { FaultService } from './Fault.service';

@Controller('openmct/faults')
export class FaultsController {
	constructor(private faultsService: FaultService) {}

	@Post('acknowledge')
	acknowledgeFault(
		@Body() { faultId, comment }: { faultId: string; comment: string },
	) {
		return this.faultsService.acknowledgeFault(faultId, comment);
	}

	@Post('shelve')
	shelveFault(
		@Body()
		{
			faultId,
			shelved,
			shelveDuration,
			comment,
		}: {
			faultId: string;
			shelved: boolean;
			shelveDuration: number;
			comment: string;
		},
	) {
		return this.faultsService.shelveFault(
			faultId,
			shelved,
			shelveDuration,
			comment,
		);
	}
}

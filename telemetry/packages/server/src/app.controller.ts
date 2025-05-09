import { Controller, Get } from '@nestjs/common';
import type { AppService } from './app.service';

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('ping')
	getPing(): string {
		return this.appService.getPing();
	}
}

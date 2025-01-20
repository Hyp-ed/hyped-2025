import * as dotenv from 'dotenv';
dotenv.config();

import { NestFactory } from '@nestjs/core';
import { WINSTON_MODULE_NEST_PROVIDER } from 'nest-winston';
import { AppModule } from './app.module';

async function bootstrap() {
	const app = await NestFactory.create(AppModule, {
		bufferLogs: true,
	});
	app.enableCors();
	app.useLogger(app.get(WINSTON_MODULE_NEST_PROVIDER));

	await app.listen(3000);
}

void bootstrap();

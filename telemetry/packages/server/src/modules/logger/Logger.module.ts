import { ENV } from '@/modules/core/config';
import { Module } from '@nestjs/common';
import {
	WinstonModule,
	type WinstonModuleOptions,
	utilities,
} from 'nest-winston';
import { format, transports } from 'winston';
import { LiveLogsTransport } from '../live-logs/LiveLogsTransport';

// In top-level 'telemetry' directory
const LOGGING_DIRECTORY = '../../logs';

const unhandledErrorFormat = format((info) => {
	if (info[Symbol.for('level')] === 'error' && info['error']) {
		const error = info['error'] as {
			name: string;
			message: string;
			stack: string;
		};
		return {
			context: error.name,
			message: error.message,
			// stack: info['error']['stack'], - not using this for now
			level: 'error',
			[Symbol.for('level')]: 'error',
		};
	}
	return info;
});

export const loggerOptions: WinstonModuleOptions = {
	level: ENV === 'development' ? 'debug' : 'info',
	format: format.combine(
		unhandledErrorFormat(),
		format.timestamp({
			format: 'YYYY-MM-DD HH:mm:ss',
		}),
		format.errors({ stack: false }),
		format.splat(),
		format.json(),
	),
	transports: [
		new LiveLogsTransport(),
		new transports.File({
			filename: 'errors.log',
			dirname: LOGGING_DIRECTORY,
			level: 'error',
		}),
		new transports.File({ filename: 'all.log', dirname: LOGGING_DIRECTORY }),
		...(ENV === 'development'
			? [
					new transports.Console({
						format: format.combine(
							format.timestamp(),
							format.ms(),
							utilities.format.nestLike('Telemetry', {
								colors: true,
								prettyPrint: true,
							}),
						),
						handleExceptions: true,
					}),
				]
			: []),
	],
	exitOnError: false,
};

const logger = WinstonModule.forRoot(loggerOptions);

@Module({
	imports: [logger],
	exports: [logger],
})
export class LoggerModule {}

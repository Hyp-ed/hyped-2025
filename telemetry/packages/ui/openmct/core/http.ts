import ky from 'ky';
import { HTTP_DEBUG, SERVER_ENDPOINT } from './config';

export const http = ky.create({
	prefixUrl: SERVER_ENDPOINT,
	hooks: {
		beforeRequest: [
			(request) => {
				// eslint-disable-next-line no-console
				if (HTTP_DEBUG) console.log('Request:', request);
			},
		],
		afterResponse: [
			(request, options, response) => {
				// eslint-disable-next-line no-console
				if (HTTP_DEBUG) console.log('Response:', response);
			},
		],
	},
});

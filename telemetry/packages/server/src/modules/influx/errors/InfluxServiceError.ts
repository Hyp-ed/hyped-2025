export class InfluxServiceError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'InfluxServiceError';
	}
}

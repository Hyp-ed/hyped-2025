export class StateUpdateValidationError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'StateUpdateValidationError';
	}
}

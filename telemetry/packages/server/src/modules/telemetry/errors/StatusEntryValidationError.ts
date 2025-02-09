export class StatusEntryValidationError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "StatusEntryValidationError";
	}
}

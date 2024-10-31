export class MeasurementReadingValidationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'MeasurementReadingValidationError';
  }
}

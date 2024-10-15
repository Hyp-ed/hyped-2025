export class MqttIngestionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'MqttIngestionError';
  }
}

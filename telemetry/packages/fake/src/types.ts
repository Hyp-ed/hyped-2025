import type { RangeMeasurement } from '@hyped/telemetry-types';
/**
 * Unique variable readinfgs each sensor provides
 * E.g. accelerometers generate values for acceleration, displacement and velocity
 */
export type LiveReading = RangeMeasurement & { readings: Readings };

export type SensorData = Record<string, LiveReading>;

/**
 * Sensor property containing values for each of its measured quantities
 */
export type Readings = {
  [measurement: string]: number;
};

/**
 * Return type for sensor class instantiation
 */
export type BaseSensor = {
  getData: (t: number) => Readings;
  getRandomData: (prevValue: number, readings: Readings) => Readings;
};

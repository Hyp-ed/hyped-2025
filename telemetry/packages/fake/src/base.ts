import type { Limits } from '@hyped/telemetry-types';
import type { LiveReading, Readings } from './types';
import { Utilities } from './utils';

export abstract class Sensor {
	// Define static objects, updated each timestep //
	// Records the actual time each sensor should be sampled next
	// This object refers to sensors' sampling times to monitor the next time for each sensors' reading (in real time)
	public static nextSamplingTimes: Record<string, number>;
	// Records whether each sensor has been sampled at the current time with a boolean flag for each
	public static isSampled: Record<string, boolean> = {};
	// Stores most recent sensor readings for all sensors, accessible by all sensors
	// Null is used to indicate that the sensor has not been sampled at the current time
	public static lastReadings: Record<string, Readings> = {};

	// Sensor properties
	readonly type: string; // sensor type (same as the name of the object in sensorData)
	readonly format: 'float' | 'integer'; // for random ternary logic (keyence is integer, rest are float)
	readonly limits: Limits;
	readonly rms_noise: number;
	readonly delta_t: number;

	// Variable sensor data
	protected time: number; // current time in seconds

	// Extract relevant properties from sensor data entries
	constructor({
		type,
		format,
		limits,
		rms_noise,
		sampling_time,
		readings,
	}: LiveReading) {
		Object.assign(this, {
			type,
			format,
			limits,
			rms_noise,
		});
		this.delta_t = sampling_time / 1000; // convert ms to s
		this.time = 0;
		// Add initial sensor values to global readings object
		Sensor.lastReadings[this.type] = readings;
	}

	/**
	 * Main data gen method shared by all sensors
	 * Returns the Readings object, filtered into the values to be published with MQTT
	 * For motion, only acceleration, velocitity and displacement are uploaded
	 * Accelerometers are used to estimate readings, then these values are
	 *   propagated to the three variables above
	 * @param t time in seconds
	 */
	abstract getData(t: number): Readings;

	getRandomData(readings: Readings): Readings {
		for (const unit in readings) {
			readings[unit] = Utilities.getRandomValue(
				readings[unit],
				this.rms_noise,
				this.format,
			);
		}
		return readings;
	}
}

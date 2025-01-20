import { Sensor } from '../base';
import type { LiveReading, Readings } from '../types';
import { Utilities as utils } from '../utils';
import { Temperature } from './temperature';

export class Resistance extends Temperature {
	private alpha = 5 * 10 ** -3; // Temperature coefficient of resistance (steel)
	private r0: number; // Initial value

	constructor(data: LiveReading) {
		super(data);
		// Set reference value and convert to ohms for higher precision output values
		this.r0 = Sensor.lastReadings.resistance.power_line_resistance * 10 ** 3;
	}

	/**
	 * Resistance can be assumed constant, seeing little variation with temperature
	 *   change.
	 * This data verifies the power line's continual safety by checking resistance
	 *   is as expected during operation.
	 */
	getData(): Readings {
		if (!Sensor.isSampled.temperature) {
			this.temp = utils.average(Object.values(super.getData()));
			Sensor.isSampled.temperature = true;
		}

		const readings = Object.keys(Sensor.lastReadings.resistance).map((key) => {
			// R = R0 * (1 + α(T - T0))
			const r = this.r0 * (1 + this.alpha * (this.temp - this.temp0));
			return [
				key,
				utils.round2DP((r + utils.gaussianRandom(this.rms_noise)) * 0.001),
			];
		});

		return Object.fromEntries(readings);
	}
}

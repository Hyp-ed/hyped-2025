import { Sensor } from '../base';
import type { LiveReading, Readings } from '../types';
import { Utilities } from '../utils';
import { Motion } from './motion';

export class Temperature extends Motion {
	protected temp: number;
	protected temp0: number;

	// Arbitrary coefficients for estimating temperature changes
	private params = {
		drag: 0.1,
		friction: 0.3,
		heatGen: 0.5,
	};

	constructor(data: LiveReading) {
		super(data);
		// Initial temp used for reference by subclass(es)
		this.temp0 = Utilities.average(Object.values(data.readings));
		this.temp = this.temp0;
	}

	getData(): Readings {
		this.temp += // Air drag and internal heat generation
			Math.pow(this.velocity, 3) * this.params.drag +
			this.velocity * this.params.heatGen;
		this.temp += // On the track, temperature increases with work done
			this.velocity < this.liftoffSpeed
				? Math.pow(this.displacement, 2) * this.params.friction
				: Math.pow(this.displacement, 2) *
					(this.liftoffSpeed / this.velocity) *
					this.displacement *
					this.params.friction;

		return Object.fromEntries(
			Object.keys(Sensor.lastReadings.temperature).map((key) => {
				return [
					key,
					Utilities.round2DP(
						this.temp + Utilities.gaussianRandom(this.rms_noise),
					),
				];
			}),
		);
	}
}

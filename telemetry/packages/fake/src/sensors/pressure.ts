import { Sensor } from '../base';
import type { LiveReading, Readings } from '../types';
import { Utilities as utils } from '../utils';
import { Temperature } from './temperature';

export class Pressure extends Temperature {
	private pResBrakes0: number;
	private pResSusp0: number;

	private airProps = {
		rho: 1.225, // air density (kg/m3)
		atm: 101325, // atmospheric pressure (Pa)
	};
	private coefficients = {
		lossFactor: 0.05, // 5% internal pressure losses
		stagnation: 0.5,
		wake: 0.25,
		brakingFactor: 0.05, // to calculate pressure increase due to braking force
	};

	constructor(data: LiveReading) {
		super(data);
		// this.prevVals = Array<number>(10).fill(0);
		this.pResBrakes0 = data.readings['pressure_brakes_reservoir'];
		this.pResSusp0 = data.readings['pressure_active_suspension_reservoir'];
	}

	getData(): Readings {
		const newData = { ...Sensor.lastReadings.pressure };
		// Pneumatic pressure gauges
		newData['pressure_front_pull'] = this.bernoulli('stagnation');
		newData['pressure_front_push'] =
			(1 - this.coefficients.lossFactor) * newData['pressure_front_pull'];
		newData['pressure_back_pull'] = this.bernoulli('wake');
		newData['pressure_back_push'] =
			(1 - this.coefficients.lossFactor) * newData['pressure_back_pull'];

		// Reservoir pressure
		newData['pressure_brakes_reservoir'] = this.idealGasLaw('brakes');
		newData['pressure_active_suspension_reservoir'] =
			this.idealGasLaw('suspension');

		// Brake pressure
		newData['pressure_front_brake'] = this.brakePressure();
		newData['pressure_back_brake'] = newData['pressure_front_brake'];

		return Object.fromEntries(
			Object.entries(newData).map(([key, value]) => {
				return [
					key,
					utils.round2DP(
						(value + utils.gaussianRandom(this.rms_noise)) * 10 ** -5, // convert back to bar
					),
				];
			}),
		);
	}

	private bernoulli(loc: 'stagnation' | 'wake'): number {
		const { rho, atm } = this.airProps;
		return atm + this.coefficients[loc] * (rho * Math.pow(this.velocity, 2));
	}

	private idealGasLaw(loc: 'brakes' | 'suspension'): number {
		this.temp +=
			this.acceleration < 0 && loc == 'brakes'
				? Math.abs(this.acceleration) * this.coefficients.brakingFactor
				: 0;
		return loc == 'brakes'
			? this.pResBrakes0 * (this.cToK(this.temp) / this.cToK(this.temp0))
			: this.pResSusp0 * (this.cToK(this.temp) / this.cToK(this.temp0));
	}

	private brakePressure(): number {
		const { atm } = this.airProps;
		const p = Sensor.lastReadings.pressure['pressure_front_brake'];
		if (this.acceleration > 0 && p > atm) {
			return p - 100; // arbitrary value for pressure drop
		} else if (this.acceleration > 0) {
			return atm;
		} else {
			this.temp +=
				Math.abs(this.acceleration) * 5 * this.coefficients.brakingFactor;
			return atm * (this.cToK(this.temp) / this.cToK(this.temp0));
		}
	}

	private cToK(temp: number): number {
		return temp + 273.15;
	}
}

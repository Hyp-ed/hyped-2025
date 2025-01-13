export class Utilities {
	/**
	 * Greatest common divisor
	 */
	public static gcd(nums: number[]): number {
		const sortedNums = nums.sort((a, b) => b - a);
		return sortedNums.reduce((acc, c) => {
			return c === 0 ? acc : Utilities.gcd([c, acc % c]);
		});
	}

	/**
	 * Simple floating point rounding method
	 * @param num
	 * @returns
	 */
	public static round2DP(num: number): number {
		return Number.parseFloat(num.toFixed(2));
	}

	/**
	 * Generates random noise value from a Gaussian distribution
	 * @param mean self-explanatory
	 * @param std_dev sensor's RMS noise value, used as the standard deviation
	 * @returns random number defined by the normal distribution of stdDev = RMS noise
	 */
	public static gaussianRandom(std_dev: number, mean = 0): number {
		// Using the Box-Muller transform to generate random values from a normal distribution
		const u1 = Math.random();
		const u2 = Math.random();
		const z = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);

		return Number.parseFloat((z * std_dev + mean).toFixed(2));
	}

	/**
	 * Generates random value from random distribution defined by provided range
	 * 99% probability of random value falling within critical limits
	 * This results in a z-score of 2.576 for confidence level of 99%
	 */
	public static getRandomValue(
		prevValue: number,
		rms_noise: number,
		format: 'float' | 'integer',
	): number {
		return format === 'float'
			? Number.parseFloat(
					Utilities.gaussianRandom(rms_noise, prevValue).toFixed(2),
				)
			: Number.parseInt(
					Utilities.gaussianRandom(rms_noise, prevValue).toFixed(2),
				);
	}

	/**
	 * Simple arithmetic mean
	 * @param values numerical sample
	 * @returns mean value
	 */
	public static average(values: number[]): number {
		return values.reduce((acc, c) => acc + c) / values.length;
	}

	/**
	 * Logistic function used as an analytical basis for dynamic variables which change over time
	 * @param t - current time
	 * @param peak - asymptotic maximum value
	 * @param k - exponential growth factor
	 * @param t0 - time of curve inflection (df²/dt² = 0)
	 * @returns f(t) - current reading according to idealised model
	 */
	public static logistic(
		t: number,
		peak: number,
		k: number, // exponential growth rate factor
		t0: number, // time at which second derivative reaches a stationary point
	): number {
		return Number.parseFloat((peak / (1 + Math.exp(-k * (t - t0)))).toFixed(2));
	}

	/**
	 * Sinusoidal damped oscillation
	 * @param t - current time
	 * @param freq - angular frequency
	 * @param phase - phase shift (angle)
	 * @param decay - exponential decay factor
	 * @param amp - oscillation peak amplitude
	 * @returns f(t)
	 */
	public static oscillateDecay(
		t: number,
		freq: number,
		phase: number,
		decay: number,
		amp: number,
	): number {
		return Number.parseFloat(
			(amp * Math.exp(-decay * t) * Math.cos(freq * t + phase)).toFixed(2),
		);
	}

	/**
	 * Gets the exponential average of a recent set of values
	 * @param vals previous values (and chosen length of array)
	 * @param alpha weighting factor
	 * @returns exponentially weighted average
	 */
	public expMovingAvg(vals: number[], alpha: number): number | undefined {
		if (alpha <= 0 || alpha > 1 || !vals.length) {
			return;
		}
		let sum = 0;
		vals.forEach((v, i) => {
			const weight = alpha ** (vals.length - 1 - i);
			sum += v * weight;
		});
		return sum / (1 - alpha ** vals.length);
	}
}

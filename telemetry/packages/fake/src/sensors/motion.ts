import { Sensor } from '../base';
import { measurements } from '../config';
import { Utilities } from '../utils';
import { LiveReading, Readings } from '../types';

export class Motion extends Sensor {
  protected displacement: number;
  protected velocity: number;
  protected acceleration: number;
  // Velocity threshold at which levitation is activated
  protected liftoffSpeed = 5;

  private logParams = {
    growth: 0.4,
    // Ensures acceleration peaks at its limiting operating value
    t_0: 12.5,
    // Max vel. set to 95% of upper limit giving a small margin for noise fluctuations
    stState: 0.95 * measurements.velocity.limits.critical.high,
  };

  constructor(accelerometer: LiveReading) {
    super(accelerometer);
    const { displacement, velocity, acceleration } = Sensor.lastReadings.motion;
    Object.assign(this, { displacement, velocity, acceleration });
  }

  getData(t: number): Readings {
    const velocityEstimate = Utilities.logistic(
      t,
      this.logParams.stState,
      this.logParams.growth,
      this.logParams.t_0,
    );

    // Use estimate to calculate accelerometer reading
    let accelerometerReading =
      (velocityEstimate - this.velocity) / this.delta_t;
    // Assert reading is not above critical limit
    accelerometerReading =
      accelerometerReading >= this.limits.critical.high
        ? this.limits.critical.high
        : accelerometerReading;
    accelerometerReading += Utilities.gaussianRandom(this.rms_noise);

    // Use trapezoidal integration to find velocity, displacement
    const avgAcceleration = (accelerometerReading + this.acceleration) / 2;
    this.velocity += avgAcceleration * this.delta_t;
    this.displacement += this.velocity * this.delta_t;
    this.acceleration = accelerometerReading;

    return {
      acceleration: Utilities.round2DP(this.acceleration),
      velocity: Utilities.round2DP(this.velocity),
      displacement: Utilities.round2DP(this.displacement),
    };
  }
}

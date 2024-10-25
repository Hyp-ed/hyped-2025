import { Magnetism } from './magnetism';
import { Sensor } from '../base';
import { LiveReading, Readings } from '../types';
import { Utilities } from '../utils';

export class Levitation extends Magnetism {
  private timeActive: number; // dynamic time variable
  private timeOffset: number; // time at which lev. is activated
  private isActive = false;

  private prevVals: number[]; // store recent values to identify reaching steady state
  private setpoint = 50; // mm
  private inSteadyState = true;
  private sse = 0.02; // steady state error (±1% of setpoint)

  private readonly logRiseParams = {
    t_0: 1.9, // inflection point
    growth: 2.5, // growth rate
    t_f: 2.225, // time when logistic function switches to sinusoidal exp. decay
    l_peak: 60, // peak amplitude
  };
  private readonly oscParams = {
    freq: 4, // rad/s
    phase: 3.7, // phase angle (rad)
    decay: 0.2, // decay rate
    amp: this.logRiseParams.l_peak - this.setpoint, // oscillation amplitude
  };
  private readonly logFallParams = {
    t_0: 2.5, // land on track within 5s
    growth: 1.5, // decline smoother than rise
  };

  constructor(data: LiveReading) {
    super(data);
    // 10 values ensures minimal error for small sampling times
    //   and sufficient reaction time for large sampling times
    this.prevVals = Array<number>(10).fill(0);
  }

  // Reset time at rising and falling stages
  initiate(t: number) {
    this.timeOffset = t;
    this.timeActive = 0;
    this.isActive = !this.isActive;
    this.inSteadyState = false;
  }

  getData(t: number): Readings {
    // Start relative timekeeping when EM powers on
    if (this.isFieldOn() && !this.isActive) this.initiate(t);

    // Keep at zero while on track
    // ToF sensor noise negated/ignored as value is fixed and known
    if (!this.isActive && this.inSteadyState)
      return Sensor.lastReadings.levitation;

    // Code below reachable after EM field first turns on

    // Update levitating time
    this.timeActive = t - this.timeOffset;

    // Rising stage
    if (this.isActive) {
      if (this.timeActive <= this.logRiseParams.t_f) {
        this.prevVals.push(
          Utilities.logistic(
            this.timeActive,
            this.logRiseParams.l_peak,
            this.logRiseParams.growth,
            this.logRiseParams.t_0,
          ),
        );
        this.prevVals.shift();
      }

      // Oscillation to steady state
      else if (
        !this.inSteadyState &&
        this.timeActive > this.logRiseParams.t_f
      ) {
        this.prevVals.push(
          this.setpoint +
            Utilities.oscillateDecay(
              this.timeActive - this.logRiseParams.t_f,
              this.oscParams.freq,
              this.oscParams.phase,
              this.oscParams.decay,
              this.oscParams.amp,
            ),
        );
        this.prevVals.shift();

        this.inSteadyState = this.prevVals.every(
          (l) => Math.abs(this.setpoint - l) < (this.sse / 2) * this.setpoint,
        );
      }

      // If at steady state, fix value to avoid unneeded computation
      else {
        this.prevVals.push(this.setpoint);
        this.prevVals.shift();
      }

      // Monitor EM field during steady state
      if (this.isFieldOn()) return this.updateReadings(this.prevVals);
      // Reset time for decline stage
      else this.initiate(t);
    }

    // Code below reachable once pod decline begins

    // Once pod slows down, begin gradual decline in levitation
    if (!this.inSteadyState) {
      this.prevVals.push(
        this.setpoint *
          (1 -
            Utilities.logistic(
              this.timeActive,
              1,
              this.logFallParams.growth,
              this.logFallParams.t_0,
            )),
      );

      this.inSteadyState = this.prevVals.every(
        (l) => Math.abs(l) < (this.sse / 2) * this.setpoint,
      );

      return this.updateReadings(this.prevVals);
    } else {
      // Once touched down, reset sensor readings to zero
      return this.updateReadings([], true);
    }
  }

  private updateReadings(prevVals: number[], onTrack = false): Readings {
    return Object.fromEntries(
      Object.keys(Sensor.lastReadings.levitation).map((key) => {
        return [
          key,
          Utilities.round2DP(
            prevVals.slice(-1)[0] +
              // Add noise if levitating, otherwise value is fixed so noise is negated
              (onTrack ? 0 : Utilities.gaussianRandom(this.rms_noise)),
          ),
        ];
      }),
    );
  }
}

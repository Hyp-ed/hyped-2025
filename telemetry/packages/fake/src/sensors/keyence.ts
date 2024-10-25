import { Motion } from './motion';
import { Sensor } from '../base';
import { LiveReading, Readings } from '../types';
import { trackLength } from '../config';

/**
 * Integer value in range [0, 16], which directly corresponds to the track distance.
 * Live data takes the form of a staircase with varying step width
 * Instead of sensor noise there is measurement tolerance
 */
export class Keyence extends Motion {
  private podLength = 2.5;

  constructor(data: LiveReading) {
    super(data);
  }

  getData(t: number): Readings {
    // Keyence sensors are evenly distributed along the pod
    // Displacement is measured at the nose of the pod
    const sensorRegion =
      this.podLength / (Object.keys(Sensor.lastReadings.keyence).length - 1);
    const noPoles = this.limits.critical.high;

    if (!Sensor.isSampled['motion']) {
      this.displacement = super.getData(t).displacement;
      Sensor.isSampled['motion'] = true;
    } else {
      this.displacement = Sensor.lastReadings.motion.displacement;
    }

    this.displacement += this.addTolerance();

    return Object.fromEntries(
      Object.keys(Sensor.lastReadings.keyence).map((key, i) => {
        const relDisp =
          this.displacement - sensorRegion * i >= 0
            ? this.displacement - sensorRegion * i
            : 0; // assert value is positive
        return [key, Math.floor(relDisp * (noPoles / trackLength))];
      }),
    );
  }

  /**
   * Keyence sensor has single-digit millimetre tolerance
   */
  addTolerance() {
    return Math.random() * 0.01 * (Math.random() >= 0.5 ? 1 : -1);
  }
}

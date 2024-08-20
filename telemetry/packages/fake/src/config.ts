import { pods } from '@hyped/telemetry-constants';
import { Pod, RangeMeasurement } from '@hyped/telemetry-types';
import { LiveReading, SensorData } from './types';

type podID = keyof typeof pods;

/**
 * Extracts and categorises relevant sensor data
 */
const filterMeasurements = (id: podID) => {
  const pod = Object.values(pods).find((pod: Pod) => pod.id === id) as Pod;
  const filteredData = {} as Record<string, RangeMeasurement>;
  //
  Object.entries(pod.measurements).forEach(([key, meas]) => {
    if (meas.format !== 'enum') {
      filteredData[key] = meas;
      filteredData[key].name = key.replace(/_[^_]*\d$/, '');
    }
  });
  return filteredData;
};

export const measurements = filterMeasurements('pod_2024');

/**
 * Gets an arbitrary initial value for each reading
 * Testing functionTo be replaced with user defined params fetched from GUI
 * @param data a key - value item from the measurements object
 * @returns initial value for a given sensor/measurement
 */
const getInitialValue = (data: RangeMeasurement): number => {
  // Define initial conditions
  const initialVals: Record<string, number> = {
    accelerometer: 0,
    acceleration: 0,
    displacement: 0,
    // velocity: measurements.velocity.limits.critical.high * 0.1, // initial velocity > 0 for continuity of logistic function
    velocity: 0.3, // m/s (this aligns closely with logistic curve y-intercept)
    pressure: data.name.endsWith('reservoir') ? 5 : 1,
    thermistor: 25,
    keyence: 0,
    hall_effect: 0,
    levitation_height: 0,
    power_line_resistance: 10,
  };

  // Set initial value based on sensor types defined above
  if (Object.prototype.hasOwnProperty.call(initialVals, data.name)) {
    return initialVals[data.name];
  } else if (data.name.startsWith('pressure')) {
    // Pressure gauges are subdivided into push, pull, brake and reservoir with different initial values
    return initialVals.pressure;
  } else {
    // If the sensor is not recognised, return a random value within the critical limits
    const { low, high } = data.limits.critical;
    return Math.floor(Math.random() * (high - low)) + low;
  }
};

/**
 * Create new object to store existing and additional sensor parameters
 * Groups sensors by data source by replacing sensor type with group's type
 */
export const sensorData: SensorData = Object.fromEntries(
  Object.values(measurements)
    .reduce(
      (acc, sensor): any => {
        if (!acc.seen) acc.seen = new Set();
        // Check if the sensor key has already been processed
        if (!acc.seen.has(sensor.type)) {
          acc.seen.add(sensor.type);
          // Add one key value pair for each sensor type
          // Each type holds all data on its constituent sensors
          acc.entries.push([sensor.type, sensor]);
        }
        return acc;
      },
      { seen: new Set(), entries: [] as [string, RangeMeasurement][] },
    )
    .entries // Set new readings property to the sensors' initial conditions
    .map(([name, data]: [string, RangeMeasurement]) => [
      name,
      {
        ...data,
        // Create object with a key-value pair for each measurement of a given sensor type
        readings: Object.fromEntries(
          Object.keys(measurements)
            .filter(
              (name) =>
                !name.endsWith('avg') && measurements[name].type == data.type,
            )
            .map((el) => [el, getInitialValue(measurements[el])]),
        ),
      } as LiveReading,
    ]),
);

// Parameter storing distance of track, once finsih point is reached program will end
export const trackLength = measurements.displacement.limits.critical.high;

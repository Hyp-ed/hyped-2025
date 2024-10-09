/**
 * Main file which initialises the generation of the data series and uploads to GUI in real time.
 * @param runTime (CLI) simulation time in ms (not real time, based on sensor timesteps)
 * @param random (CLI) option to simulate random data - later to be replaced with a config object
 * which allows user to randomise select sensor readings. Default is false
 * @param specific (CLI) an array of specific sensor readings to simulate. Default is false
 * i.e. simulate all sensors
 */
import { SensorManager } from './sensorManager';
import { sensorData } from './config';

const args = process.argv.slice(2);
const shouldRandomise = args.includes('--random') ? true : false;

// Filter for user-defined specific sensors, otherwise simulate all
Object.keys(sensorData).filter((sensor) => args.includes(sensor));

// Ensure input sensor options are valid and format them appropriately
const sensorsToRun = args.includes('--specific')
  ? args
      .slice(args.indexOf('--specific') + 1)
      .map((s: string) => s.toLowerCase())
      .filter((s: string) =>
        Object.prototype.hasOwnProperty.call(sensorData, s),
      )
  : Object.keys(sensorData);

// Instantiate sensor manager
const sensorMgmt = new SensorManager(sensorsToRun);

// Simulates sensor readings and uploads to the server
sensorMgmt.generateData(shouldRandomise);

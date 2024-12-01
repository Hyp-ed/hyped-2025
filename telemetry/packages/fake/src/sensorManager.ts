import MQTT from 'mqtt';
import { Sensor } from './base';
import { sensorData, trackLength } from './config';
import { type SensorInstance, sensors } from './sensors/index';
import type { Readings } from './types';
import { Utilities as utils } from './utils';

export class SensorManager {
	// Create array to store sensor instances
	private sensors: SensorInstance<(typeof sensors)[keyof typeof sensors]>[] =
		[];
	// Record the sampling intervals for each sensor
	private samplingTimes: Record<string, number> = {};
	// Global clock for simulation runtime
	private globalTime = 0;
	// Mqtt client
	private client: MQTT.MqttClient;

	/**
	 * The sensors form a hierarchical dependency tree
	 * At the top level is Motion, which relies only on time
	 * All other sensor data relies on motion readings, either
	 *  directly or as a grandchild of the motion class
	 * Key = sensor
	 * Value = parent class
	 */
	private dependencies: Record<string, string | null> = {
		motion: null,
		keyence: 'motion',
		temperature: 'motion',
		pressure: 'temperature',
		resistance: 'temperature',
		magnetism: 'motion',
		levitation: 'magnetism',
	};

	/**
	 * Sensor manager singleton class
	 * Controls and connects all sensor classes
	 * @param sensorsToRun user-defined array of sensor names to be run in the current simulation
	 */
	constructor(private sensorsToRun: string[]) {
		// Create sensor instances
		this.instantiateSensors(this.sensorsToRun);
		// Record fixed sampling time periods
		for (const s of this.sensorsToRun) {
			this.samplingTimes[s] = sensorData[s].sampling_time;
		}
		// Initialize MQTT connection
		this.client = MQTT.connect('MQTT://mosquitto:1883');
	}

	/**
	 * Runs transient data generation
	 * Updates global variables each iteration
	 * End program once runTime has been reached
	 * @param random boolean input set by user, allows for completely random data
	 */
	public generateData(random = false): void {
		// Calculate base sampling interval using lowest common divisor of all sensors' sampling periods
		const interval = utils.gcd(Object.values(this.samplingTimes));

		const simulationInterval = setInterval(() => {
			// Reset all 'sampled' flags to false
			this.resetSampledState();
			for (const sensor of this.sensors) {
				// Generate data if current time corresponds to sensor's sampling time
				if ((this.globalTime / 1000) % sensor.delta_t === 0) {
					// Get the sensors' output data
					const readings: Readings = !random
						? sensor.getData(this.globalTime / 1000) // convert time to seconds for calculations
						: sensor.getRandomData(Sensor.lastReadings[sensor.type]);
					// Store latest readings and set sensors' sampled state to true
					Sensor.lastReadings[sensor.type] = readings;
					Sensor.isSampled[sensor.type] = true;

					// Publish sensor readings under the topic of
					//   each and for each measurement key to the data broker
					for (const [measurement, value] of Object.entries(readings)) {
						this.publishData(measurement, value.toString());
					}
				}
			}

			// Implement exit condition
			if (Sensor.lastReadings.motion.displacement >= trackLength) {
				clearInterval(simulationInterval);
				this.generateData(random);
			}

			this.globalTime += interval;
		}, interval);
	}

	/**
	 * Instantiate sensors and their required superclasses and store instances in array
	 */
	private instantiateSensors(sensorsToRun: string[]): void {
		// Record sensors names to be added to instances array
		const activeSensors: Set<string> = new Set();
		// Function to activate all sensors required
		const getActiveSensors = (name: string): void => {
			if (this.dependencies[name] == null) {
				activeSensors.add(name);
			} else {
				getActiveSensors(this.dependencies[name] as string);
				activeSensors.add(name);
			}
			return;
		};
		// Populate set
		for (const s of sensorsToRun) {
			getActiveSensors(s);
		}
		// Define sensor instances
		// Correct sorting is automatic as recursion forces all parent class sensors to be
		//   added before their inheriting classes
		for (const s of Array.from(activeSensors)) {
			this.sensors.push(new sensors[s](sensorData[s]));
		}
	}

	/**
	 * Reset all sensors' isSampled flags to false on each iteration
	 */
	private resetSampledState(): void {
		for (const sensor of Object.keys(Sensor.isSampled)) {
			Sensor.isSampled[sensor] = false;
		}
	}

	/**
	 * Uploads data through MQTT broker to the frontend
	 * The properties of the sensors' readings objects are the keys which are appended to the topic path, i.e. ...measurements/[key]
	 * So simply append the key and publish the value as the payload
	 * Subscribed clients extract values using payload[measurementKey]
	 */
	private publishData(measurement: string, reading: string): void {
		this.client.publish(`hyped/pod_2024/measurement/${measurement}`, reading, {
			qos: 1,
		});
	}
}

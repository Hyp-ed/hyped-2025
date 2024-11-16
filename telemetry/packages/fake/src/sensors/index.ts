import { Keyence } from './keyence';
import { Levitation } from './levitation';
import { Magnetism } from './magnetism';
// Individual sensor classes
import { Motion } from './motion';
import { Pressure } from './pressure';
import { Resistance } from './resistance';
import { Temperature } from './temperature';

type SensorType =
	| typeof Motion
	| typeof Keyence
	| typeof Pressure
	| typeof Temperature
	| typeof Resistance
	| typeof Magnetism
	| typeof Levitation;

// Instance type for sensor classes
export type SensorInstance<T extends new (...args: any[]) => any> =
	InstanceType<T>;

// Export object containing all sensor classes
export const sensors = {
	motion: Motion,
	keyence: Keyence,
	temperature: Temperature,
	resistance: Resistance,
	pressure: Pressure,
	magnetism: Magnetism,
	levitation: Levitation,
} as Record<string, SensorType>;

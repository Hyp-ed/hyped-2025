import type { MeasurementLimits } from "../pods/pods";

/**
 * Type of an Open MCT measurement.
 */
export type OpenMctMeasurement = {
	name: string;
	key: string;
	type: string;
	values: {
		key: string;
		name: string;
		unit?: string;
		format: string;
		min?: number;
		max?: number;
		limits?: MeasurementLimits;
		enumerations?: {
			value: number;
			string: string;
		}[];
		hints?: {
			range?: number;
			domain?: number;
		};
		source?: string;
		units?: {
			domain: string;
		};
	}[];
};

/**
 * Type of an Open MCT pod.
 */
export type OpenMctPod = {
	id: string;
	name: string;
	measurements: OpenMctMeasurement[];
};

/**
 * Type of an Open MCT dictionary.
 */
export type OpenMctDictionary = Record<string, OpenMctPod>;

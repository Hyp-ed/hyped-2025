import type { OpenMctMeasurement } from '@hyped/telemetry-types';
import type { OpenMCT } from 'openmct/dist/openmct';
import type CompositionProvider from 'openmct/dist/src/api/composition/CompositionProvider';
import type { DomainObject } from 'openmct/dist/src/api/objects/ObjectAPI';
import type { ObjectIdentitifer } from '../types/ObjectIdentifier';
import { fetchObjectTypes } from './data/object-types-data';
import { fetchMeasurement, fetchPod, fetchPodIds } from './data/pods-data';
import { convertNamespaceToPodId } from './utils/convertNamespaceToPodId';

/**
 * Creates a folder for a pod.
 * @see https://github.com/nasa/openmct/blob/master/API.md#object-providers
 */
const podObjectProvider = {
	get: async (identifier: ObjectIdentitifer) => {
		const pod = await fetchPod(identifier.key);
		return {
			identifier,
			name: pod.name,
			type: 'folder',
			location: 'ROOT',
		};
	},
};

/**
 * Create the measurements for a pod.
 */
const measurementsObjectProvider = {
	get: async (identifier: ObjectIdentitifer) => {
		const podId = convertNamespaceToPodId(identifier.namespace);
		const measurement = await fetchMeasurement(podId, identifier.key);
		if (!measurement) {
			throw new Error('Measurement not found');
		}
		const telemetryValue = measurement.values.find((m) => m?.key === 'value');
		if (!telemetryValue) {
			throw new Error('Measurement does not have a telemetry source value');
		}
		return {
			identifier,
			podId,
			name: measurement.name,
			type: `hyped.${measurement.type}`,
			telemetry: {
				values: measurement.values,
			},
			location: `hyped.taxonomy:${podId}`,
			limits: telemetryValue.limits,
		};
	},
};

/**
 * This composition provider defines the list of objects (measurements) that are
 * contained within a folder (pod).
 * @see https://github.com/nasa/openmct/blob/master/API.md#composition-providers
 */
const compositionProvider = {
	/**
	 * This function is called when Open MCT needs to know if this provider
	 * applies to a particular domain object.
	 * @param domainObject The domain object to check.
	 * @returns `true` if the provider is in the root namespace and the object is a folder, `false` otherwise.
	 */
	appliesTo: (domainObject: DomainObject) => {
		return (
			domainObject.identifier.namespace === 'hyped.taxonomy' &&
			domainObject.type === 'folder'
		);
	},
	/**
	 * Loads the composition for a domain object.
	 * @param domainObject The domain object to load the composition for.
	 * @returns The list of objects that are contained within the domain object.
	 */
	load: async (domainObject: DomainObject) => {
		const podId = domainObject.identifier.key;
		const pod = await fetchPod(podId);
		return pod.measurements.map((measurement: OpenMctMeasurement) => ({
			namespace: `hyped.${podId}`,
			key: measurement.key,
		}));
	},
};

/**
 * This plugin is responsible for creating the dictionary of pods and measurements.
 * @returns
 */
export function DictionaryPlugin() {
	return async function install(openmct: OpenMCT) {
		fetchPodIds()
			.then(({ ids }) => {
				for (const podId of ids) {
					openmct.objects.addRoot(
						{
							namespace: 'hyped.taxonomy',
							key: podId,
						},
						openmct.priority.HIGH,
					);
					openmct.objects.addProvider(
						`hyped.${podId}`,
						measurementsObjectProvider,
					);
				}

				openmct.objects.addProvider('hyped.taxonomy', podObjectProvider);
			})
			.catch(() => {
				throw new Error('Failed to load dictionary');
			});

		const objectTypes = await fetchObjectTypes();
		for (const objectType of objectTypes) {
			openmct.types.addType(`hyped.${objectType.id}`, {
				name: objectType.name,
				cssClass: objectType.icon,
				...(objectType.description && {
					description: objectType.description,
				}),
			});
		}

		openmct.composition.addProvider(compositionProvider as CompositionProvider);
	};
}

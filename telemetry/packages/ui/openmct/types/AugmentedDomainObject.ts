import type { Limits } from '@hyped/telemetry-types/dist/pods/pods.types';
import type { DomainObject } from 'openmct/dist/src/api/objects/ObjectAPI';

export type AugmentedDomainObject = DomainObject & {
	podId: string;
	limits?: Limits;
};

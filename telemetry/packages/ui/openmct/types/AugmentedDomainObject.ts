import { Limits } from '@hyped/telemetry-types/dist/pods/pods.types';
import { DomainObject } from 'openmct/dist/src/api/objects/ObjectAPI';

export type AugmentedDomainObject = DomainObject & {
  podId: string;
  limits?: Limits;
};

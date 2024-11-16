import type { OpenMctObjectTypes } from '@hyped/telemetry-types';
import { http } from '../../core/http';

/**
 * Fetches all object types from the server.
 * These object types define the different types of objects (a.k.a measurements) that can be displayed in Open MCT (e.g. velocity, acceleration, etc.)
 * @returns The object types (in our own format).
 */
export const fetchObjectTypes = async () =>
	http.get('openmct/object-types').json<OpenMctObjectTypes>();

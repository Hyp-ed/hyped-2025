import { OpenMctMeasurement, OpenMctPod } from '@hyped/telemetry-types';
import { http } from '../../core/http';

/**
 * Fetches all pod IDs from the server.
 * @returns
 */
export const fetchPodIds = () =>
  http.get('openmct/dictionary/pods').json<{ ids: string[] }>();

/**
 * Fetches a pod from the server.
 * @param id The pod ID.
 * @returns The pod object in the format that Open MCT expects.
 */
export const fetchPod = (id: string) =>
  http.get(`openmct/dictionary/pods/${id}`).json<OpenMctPod>();

/**
 * Fetches a pod measurement from the server.
 * @param podId The pod ID.
 * @param measurementKey The measurement key.
 * @returns The measurement object in the format that Open MCT expects.
 */
export const fetchMeasurement = (podId: string, measurementKey: string) =>
  http
    .get(`openmct/dictionary/pods/${podId}/measurements/${measurementKey}`)
    .json<OpenMctMeasurement>();

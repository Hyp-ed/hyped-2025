import type { LevitationHeightResponse } from '@hyped/telemetry-types';
import type {
	DisplacementResponse,
	LaunchTimeResponse,
	VelocityResponse,
} from '@hyped/telemetry-types/dist/server/responses';

/**
 * The server endpoint for fetching public data.
 */
export const SERVER_ENDPOINT = `${process.env.NEXT_PUBLIC_TELEMETRY_SERVER}/pods/${process.env.NEXT_PUBLIC_POD_ID}/public-data`;

const ONE_MINUTE = 60;

/**
 * Gets the last `prev` seconds of displacement data.
 * @param prev The number of seconds to get.
 * @returns The historical displacement data.
 */
export const getDisplacement = async (
	prev: number = ONE_MINUTE,
): Promise<DisplacementResponse> => {
	const now = new Date().getTime();
	const start = now - prev * 1000;
	const response = await fetch(
		`${SERVER_ENDPOINT}/displacement?start=${start}`,
	);
	if (response.status !== 200) throw new Error('Failed to fetch displacement');
	return response.json() as Promise<DisplacementResponse>;
};

/**
 * Gets the last `prev` seconds of levitation height data.
 * @param prev The number of seconds to get.
 * @returns The historical levitation height data.
 */
export const getLevitationHeight = async (
	prev: number = ONE_MINUTE,
): Promise<LevitationHeightResponse> => {
	const now = new Date().getTime();
	const start = now - prev * 1000;
	const response = await fetch(
		`${SERVER_ENDPOINT}/levitation-height?start=${start}`,
	);
	if (response.status !== 200)
		throw new Error('Failed to fetch levitation height');
	return response.json() as Promise<LevitationHeightResponse>;
};

/**
 * Gets the last `prev` seconds of velocity data.
 * @param prev The number of seconds to get.
 * @returns The historical velocity data.
 */
export const getVelocity = async (
	prev: number = ONE_MINUTE,
): Promise<VelocityResponse> => {
	const now = new Date().getTime();
	const start = now - prev * 1000;
	const response = await fetch(`${SERVER_ENDPOINT}/velocity?start=${start}`);
	if (response.status !== 200) throw new Error('Failed to fetch velocity');
	return response.json() as Promise<VelocityResponse>;
};

/**
 * Gets the launch time of the pod (in milliseconds).
 * @returns The launch time of the pod.
 */
export const getLaunchTime = async (): Promise<LaunchTimeResponse> => {
	const response = await fetch(`${SERVER_ENDPOINT}/launch-time`);
	if (response.status !== 200) throw new Error('Failed to fetch launch time');
	return response.json() as Promise<LaunchTimeResponse>;
};

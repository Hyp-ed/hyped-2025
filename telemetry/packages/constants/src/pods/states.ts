export type PodStateType = keyof typeof ALL_POD_STATES;

export const FAILURE_STATES = {
	FAILURE_BRAKING: 'FAILURE_BRAKING',
} as const;

export const PASSIVE_STATES = {
	IDLE: 'IDLE',
	CALIBRATE: 'CALIBRATE',
	SAFE: 'SAFE',
} as const;

export const ACTIVE_STATES = {
	PRECHARGE: 'PRECHARGE',
	READY_FOR_LEVITATION: 'READY_FOR_LEVITATION',
	BEGIN_LEVITATION: 'BEGIN_LEVITATION',
	READY_FOR_LAUNCH: 'READY_FOR_LAUNCH',
	ACCELERATE: 'ACCELERATE',
	LIM_BRAKE: 'LIM_BRAKE',
	FRICTION_BRAKE: 'FRICTION_BRAKE',
	STOP_LEVITATION: 'STOP_LEVITATION',
	STOPPED: 'STOPPED',
	BATTERY_RECHARGE: 'BATTERY_RECHARGE',
	CAPACITOR_DISCHARGE: 'CAPACITOR_DISCHARGE',
} as const;

export const NULL_STATES = {
	UNKNOWN: 'UNKNOWN',
} as const;

export const ALL_POD_STATES = {
	...FAILURE_STATES,
	...PASSIVE_STATES,
	...ACTIVE_STATES,
	...NULL_STATES,
};

export const ALL_POD_STATE_TYPES = [
	'FAILURE',
	'PASSIVE',
	'ACTIVE',
	'NULL',
] as const;

export type PodStateCategoryType = (typeof ALL_POD_STATE_TYPES)[number];

export const getStateType = (
	state: string,
): (typeof ALL_POD_STATE_TYPES)[number] => {
	if (FAILURE_STATES[state as keyof typeof FAILURE_STATES]) return 'FAILURE';
	if (PASSIVE_STATES[state as keyof typeof PASSIVE_STATES]) return 'PASSIVE';
	if (ACTIVE_STATES[state as keyof typeof ACTIVE_STATES]) return 'ACTIVE';
	if (NULL_STATES[state as keyof typeof NULL_STATES]) return 'NULL';
	throw new Error(`Unknown state: ${state}`);
};

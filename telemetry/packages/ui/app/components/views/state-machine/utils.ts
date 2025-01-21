import {
	ACTIVE_STATES,
	ALL_POD_STATES,
	FAILURE_STATES,
	MODE_EXCLUDED_STATES,
	type ModeType,
	PASSIVE_STATES,
	type PodStateType,
} from '@hyped/telemetry-constants';

/**
 * Returns the node type based on the state of the pod (active, failure, static)
 * @param state The PodState of the pod
 * @returns The state-corresponding node type
 */
export const getNodeType = (state: PodStateType) => {
	if (state in FAILURE_STATES) return 'FailureNode';
	if (state in PASSIVE_STATES) return 'PassiveNode';
	if (state in ACTIVE_STATES) return 'ActiveNode';
};

export const getEnabledStates = (mode: ModeType) => {
	return Object.values(ALL_POD_STATES).filter(
		(state) => !(state in MODE_EXCLUDED_STATES[mode]),
	);
};

export const isEnabledState = (mode: ModeType, state: PodStateType) =>
	!MODE_EXCLUDED_STATES[mode].includes(state);

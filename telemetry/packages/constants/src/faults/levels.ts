export const FAULT_LEVEL = {
	WATCH: 'WATCH',
	WARNING: 'WARNING',
	CRITICAL: 'CRITICAL',
} as const;

export type FaultLevel = (typeof FAULT_LEVEL)[keyof typeof FAULT_LEVEL];

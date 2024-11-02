import { ALL_POD_STATES } from '@hyped/telemetry-constants';

export const isValidState = (state: string) => {
  return ALL_POD_STATES[state as keyof typeof ALL_POD_STATES] !== undefined;
};

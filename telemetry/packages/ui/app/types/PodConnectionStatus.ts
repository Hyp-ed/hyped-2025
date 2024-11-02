export const POD_CONNECTION_STATUS = {
  CONNECTED: 'CONNECTED',
  DISCONNECTED: 'DISCONNECTED',
  ERROR: 'ERROR',
  UNKNOWN: 'UNKNOWN',
} as const;

export type PodConnectionStatusType =
  (typeof POD_CONNECTION_STATUS)[keyof typeof POD_CONNECTION_STATUS];

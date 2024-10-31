export type RealtimeTelemetryListenerCallback = (data: {
  id: string;
  value: number;
  timestamp: number;
}) => void;

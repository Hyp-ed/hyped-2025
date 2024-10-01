export type RawLevitationHeight = {
  id: string;
  timestamp: number;
  value: string;
};

export type LevitationHeight = {
  id: string;
  timestamp: number;
  value: number;
};

export type LevitationHeightResponse = Record<string, LevitationHeight>;

export type LaunchTimeResponse = {
  launchTime: number | null;
};

export type StateResponse = {
  currentState: {
    state: string;
    timestamp: number;
    stateType: string;
  } | null;
  previousState: {
    state: string;
    timestamp: number;
    stateType: string;
  } | null;
};

export type HistoricalValueResponse = {
  id: string;
  timestamp: number;
  value: string;
}[];

export type VelocityResponse = HistoricalValueResponse;
export type DisplacementResponse = HistoricalValueResponse;

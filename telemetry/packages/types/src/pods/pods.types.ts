// common properties shared by all response variables
export type BaseMeasurement = {
  name: string;
  key: string;
  unit: string;
  type: string;
};

// range limits not to be exceeded
// some give warnings when reaching range limits
export type Limits = {
  warning?: {
    low: number;
    high: number;
  };
  critical: {
    low: number;
    high: number;
  };
};

// For numerical sensor readings described by operational range sampling parameters
export type RangeMeasurement = BaseMeasurement & {
  format: 'float' | 'integer';
  limits: Limits;
  rms_noise: number;
  sampling_time: number;
};

// For discrete status measurements with enumerated states
export type EnumMeasurement = BaseMeasurement & {
  format: 'enum';
  enumerations: {
    value: number;
    string: string;
  }[];
};

// export type Measurement as union
export type Measurement = RangeMeasurement | EnumMeasurement;

// create Pod type
export type Pod = {
  name: string;
  id: string;
  measurements: Record<string, Measurement>;
  // Not ideal given this is defined in the constants package but will do until TOML is done
  operationMode: 'ALL_SYSTEMS_ON' | 'LEVITATION_ONLY' | 'LIM_ONLY';
};

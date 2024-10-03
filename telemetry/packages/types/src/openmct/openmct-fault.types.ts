export type OpenMctFault = {
  type: null | string;
  fault: {
    acknowledged: boolean;
    currentValueInfo: {
      value: number;
      rangeCondition: string;
      monitoringResult: string;
    };
    id: string;
    name: string;
    namespace: string;
    seqNum: number;
    severity: string;
    shelved: boolean;
    shortDescription: string;
    triggerTime: string;
    triggerValueInfo: {
      value: number;
      rangeCondition: string;
      monitoringResult: string;
    };
  };
};

export type HistoricalFaults = {
  faultId: string;
  timestamp: number;
  openMctFault: OpenMctFault;
  podId: string;
  measurementKey: string;
}[];

export type OpenMctHistoricalFaults = {
  timestamp: number;
  fault: OpenMctFault;
}[];

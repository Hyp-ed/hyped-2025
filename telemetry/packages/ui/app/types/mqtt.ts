export type QoS = 0 | 1 | 2;

export type MqttPublish = (
  topic: string,
  payload: string,
  podId: string,
) => void;
export type MqttSubscribe = (topic: string, podId: string) => void;
export type MqttUnsubscribe = (topic: string, podId: string) => void;

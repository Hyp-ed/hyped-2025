import { log } from '@/lib/logger';
import { http } from 'openmct/core/http';
import toast from 'react-hot-toast';

/**
 * Defines the controls that can be sent to a pod.
 */
export const CONTROLS = {
  START: 'start',
  STOP: 'stop',
  LEVITATE: 'levitate',
  STOP_LEVITATING: 'stop-levitating',
  CLAMP: 'clamp',
  RETRACT: 'retract',
  // The below are probably deprecated for this year's pod
  START_HP: 'start-hp',
  STOP_HP: 'stop-hp',
  RAISE: 'raise',
  LOWER: 'lower',
  TILT: 'tilt',
} as const;

export type Control = (typeof CONTROLS)[keyof typeof CONTROLS];

/**
 * Sends a control message to a pod.
 * @param podId The ID of the pod to send the control to.
 * @param control The control message to send.
 */
export const sendControlMessage = async (podId: string, control: Control) => {
  log(`Sending control ${control} to pod ${podId}`, podId);
  toast(`Sending control ${control} to pod ${podId}`);
  const url = `pods/${podId}/controls/${control}`;
  await http.post(url);
};

import { PodState } from '@/components/shared/pod-state';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useMQTT } from '@/context/mqtt';
import { ALL_POD_STATES, PodStateType } from '@hyped/telemetry-constants';
import { useState } from 'react';

/**
 * A pod state updater component which allows us to set the state of a pod for testing/debug purposes. Used in the debug view.
 * @param podId The ID of the pod to update the state of.
 * @returns A component to update the state of a pod.
 */
export const PodStateUpdater = ({ podId }: { podId: string }) => {
  const [podState, setPodState] = useState<PodStateType>(ALL_POD_STATES.IDLE);
  const { publish } = useMQTT();

  /**
   * Publishes the pod state to the MQTT broker.
   */
  const publishPodState = () => {
    publish(`state`, podState, podId);
  };

  return (
    <div className="flex gap-8 w-full justify-center">
      <div className="min-w-max">
        <PodState podId={podId} />
      </div>
      <div className="space-y-2 w-[300px] my-auto">
        <Label htmlFor="pod-select">Set State:</Label>
        <div className="flex gap-2">
          <Select
            value={podState}
            onValueChange={(value) => setPodState(value as PodStateType)}
          >
            <SelectTrigger id="pod-select" className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <PodStateOptions />
            </SelectContent>
          </Select>
          <Button onClick={publishPodState}>Set</Button>
        </div>
      </div>
    </div>
  );
};

/**
 * Returns the pod state options for the pod state updater.
 */
const PodStateOptions = () =>
  Object.keys(ALL_POD_STATES).map((state) => (
    <SelectItem key={state} value={state}>
      {state}
    </SelectItem>
  ));

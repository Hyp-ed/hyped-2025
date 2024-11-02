import { usePod } from '@/context/pods';
import { cn } from '@/lib/utils';
import {
  ACTIVE_STATES,
  FAILURE_STATES,
  NULL_STATES,
  PASSIVE_STATES,
  PodStateCategoryType,
} from '@hyped/telemetry-constants';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { CircleDashed } from 'lucide-react';

/**
 * Defines the styling for each pod state.
 */
export const styles: Record<PodStateCategoryType, string> = {
  ACTIVE: 'bg-green-700 border-2 border-green-900 text-white',
  FAILURE: 'bg-red-700 border-2 border-red-900 text-white',
  PASSIVE: 'bg-gray-600 border-2 border-gray-800 text-white',
  NULL: '',
};

/**
 * Displays the current state of a pod.
 * @param podId The ID of the pod to display the state of.
 * @returns A Card component displaying the pod state.
 */
export const PodState = ({ podId }: { podId: string }) => {
  const { podState: state, name } = usePod(podId);

  return (
    <Card className="border-none">
      <CardHeader>
        <CardTitle className="flex gap-2">
          <CircleDashed /> Pod State
        </CardTitle>
        <CardDescription>The current state of {name}</CardDescription>
      </CardHeader>
      <CardContent>
        <p
          className={cn(
            'px-3 py-2 rounded-md max-w-max justify-start font-bold uppercase',
            state in ACTIVE_STATES && styles.ACTIVE,
            state in FAILURE_STATES && styles.FAILURE,
            state in PASSIVE_STATES && styles.PASSIVE,
            state in NULL_STATES && styles.NULL,
          )}
        >
          {state}
        </p>
      </CardContent>
    </Card>
  );
};

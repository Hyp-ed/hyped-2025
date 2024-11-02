import { POD_IDS, PodId, pods } from '@hyped/telemetry-constants';
import { Label } from '../ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select';
import { useCurrentPod } from '@/context/pods';

/**
 * A pod selector component which allows us to select a pod to view/control.
 */
export const PodSelector = () => {
  const { currentPod, setCurrentPod } = useCurrentPod();

  return (
    <div className="space-y-2">
      <Label htmlFor="pod-select">Select Pod:</Label>
      <Select
        value={getDisplayText(currentPod)}
        onValueChange={(v) => setCurrentPod(getPodIdFromDisplayText(v))}
      >
        <SelectTrigger id="pod-select" className="w-full">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <PodOptions />
        </SelectContent>
      </Select>
    </div>
  );
};

/**
 * Returns the pod options for the pod selector by getting the display text for each pod in the `pods.ts` file.
 */
const PodOptions = () => {
  // Get the display text for each pod in the `pods.ts` file
  const podOptions = POD_IDS.map((podId) => getDisplayText(podId));

  return podOptions.map((podOption) => (
    <SelectItem key={podOption} value={podOption}>
      {podOption}
    </SelectItem>
  ));
};

/**
 * Gets the text to display in the pod selector.
 */
const getDisplayText = (podId: PodId) => `${pods[podId].name} (${podId})`;

/**
 * Opposite of `getDisplayText()`. Gets the pod ID from the display text.
 */
const getPodIdFromDisplayText = (displayText: string) => {
  const podId = displayText.split('(')[1].split(')')[0];
  return podId as PodId;
};

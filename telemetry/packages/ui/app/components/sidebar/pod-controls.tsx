import { Button } from '@/components/ui/button';
import { CONTROLS, sendControlMessage } from '@/lib/controls';
import { cn } from '@/lib/utils';
import { ArrowUpFromLine, Rocket, Siren } from 'lucide-react';
import { SetLevitationHeight } from '../shared/set-levitation-height';
import { useState } from 'react';
import { Switch } from '../ui/switch';
import { Label } from '@radix-ui/react-label';

/**
 * Displays the pod controls.
 * @param podId The ID of the pod to display the controls of.
 * @param show Whether or not to show the controls. This is used to keep the state of the controls when the podId changes (rather than unmounting and remounting the component).
 * @returns The pod controls.
 */
export const PodControls = ({
  podId,
  show,
}: {
  podId: string;
  show: boolean;
}) => {
  return (
    <div className={cn('mt-2 space-y-8', show ? 'block' : 'hidden')}>
      <div className="flex flex-col gap-2">
        <div className="flex flex-col gap-2">
          <SetLevitationHeight podId={podId} />
          <LevitateButton podId={podId} />
        </div>
        <LaunchButton podId={podId} />
        <EmergencyStopButton podId={podId} />
      </div>
    </div>
  );
};

const LevitateButton = ({ podId }: { podId: string }) => (
  <Button
    className={cn(
      'px-2 py-6 rounded-md shadow-lg transition text-white font-bold flex gap-2',
      'bg-blue-600 hover:bg-blue-700',
    )}
    onClick={() => {
      void sendControlMessage(podId, CONTROLS.LEVITATE);
    }}
  >
    <ArrowUpFromLine /> LEVITATE
  </Button>
);

const LaunchButton = ({ podId }: { podId: string }) => {
  const [enabled, setEnabled] = useState(false);

  return (
    <div className="space-y-2 w-full">
      {/* This switch is used to enable the launch button */}
      <div className="flex items-center space-x-2">
        <Switch id="enable" checked={enabled} onCheckedChange={setEnabled} />
        <Label htmlFor="enable">Enable</Label>
      </div>
      <Button
        className={cn(
          'w-full px-2 py-6 rounded-md shadow-lg transition text-white font-bold flex gap-2',
          'bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed',
        )}
        disabled={!enabled}
        onClick={() => void sendControlMessage(podId, CONTROLS.START)}
      >
        <Rocket /> LAUNCH
      </Button>
    </div>
  );
};

export const EmergencyStopButton = ({
  podId,
  className,
}: {
  podId: string;
  className?: string;
}) => (
  <Button
    className={cn(
      'px-2 py-6 rounded-md shadow-lg transition text-white font-bold flex gap-2',
      'bg-red-700 hover:bg-red-800',
      className,
    )}
    onClick={() => void sendControlMessage(podId, CONTROLS.STOP)}
  >
    <Siren /> EMERGENCY STOP
  </Button>
);

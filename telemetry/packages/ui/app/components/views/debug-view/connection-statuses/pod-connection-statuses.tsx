import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { usePod } from '@/context/pods';
import { PodConnectionStatusType } from '@/types/PodConnectionStatus';
import { PodId, pods } from '@hyped/telemetry-constants';
import { LatencyChart } from '@/components/shared/latency-chart';
import { TrainFront } from 'lucide-react';

export const PodConnectionStatus = ({ podId }: { podId: PodId }) => {
  const { connectionStatus, latency, previousLatencies } = usePod(podId);

  // Maps the connection statuses to React components
  const statusComponentMap: Record<PodConnectionStatusType, React.ReactNode> = {
    CONNECTED: (
      <div className="flex gap-2 items-center">
        <div className="w-2 h-2 rounded-full bg-green-500 animate-[pulse_linear_1s_infinite]" />
        <p className="text-sm italic text-green-500">Connected</p>
      </div>
    ),
    DISCONNECTED: (
      <div className="flex gap-2 items-center">
        <div className="w-2 h-2 rounded-full bg-red-500" />
        <p className="text-sm italic text-red-500">Disconnected</p>
      </div>
    ),
    UNKNOWN: (
      <div className="flex gap-2 items-center">
        <div className="w-2 h-2 rounded-full bg-white animate-[pulse_linear_2s_infinite]" />
        <p className="text-sm italic text-white">Unknown</p>
      </div>
    ),
    ERROR: (
      <div className="flex gap-2 items-center">
        <div className="w-2 h-2 rounded-full bg-red-500 animate-[pulse_linear_0.2s_infinite]" />
        <p className="text-sm italic text-red-500">Error</p>
      </div>
    ),
  };

  const podName = pods[podId].name;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex gap-2">
          <TrainFront />
          {podName}
        </CardTitle>
        <CardDescription>GUI connection to {podName}</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex flex-col gap-2">
          {statusComponentMap[connectionStatus]}
          <p className="text-sm">
            Latency: {latency ? <b>{latency}ms</b> : 'N/A'}
          </p>
          <LatencyChart data={previousLatencies} />
        </div>
      </CardContent>
      <CardFooter className="text-sm text-muted-foreground">{podId}</CardFooter>
    </Card>
  );
};

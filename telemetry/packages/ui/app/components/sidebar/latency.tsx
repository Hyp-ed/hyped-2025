import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from '@/components/ui/tooltip';
import { usePod } from '@/context/pods';
import { cn } from '@/lib/utils';
import { POD_CONNECTION_STATUS } from '@/types/PodConnectionStatus';
import { LatencyChart } from '../shared/latency-chart';

/**
 * Displays the latency between the base station (GUI) and the pod.
 */
export const Latency = ({ podId }: { podId: string }) => {
	const { latency, connectionStatus, previousLatencies } = usePod(podId);

	return (
		<TooltipProvider>
			<Tooltip delayDuration={300}>
				<TooltipTrigger className="text-left">
					{connectionStatus === POD_CONNECTION_STATUS.CONNECTED && latency ? (
						<p>
							<span className="">Latency: </span>
							<span
								className={cn(
									'text-sm',
									latency <= 50 && 'text-green-500',
									latency > 50 && 'text-orange-500',
									latency > 100 && 'text-red-500',
								)}
							>
								{latency} ms
							</span>
						</p>
					) : (
						<p>
							Latency: <span className="text-red-500">N/A</span>
						</p>
					)}
				</TooltipTrigger>
				<TooltipContent
					side="bottom"
					className="border-none bg-black shadow-2xl"
				>
					<LatencyChart data={previousLatencies} />
				</TooltipContent>
			</Tooltip>
		</TooltipProvider>
	);
};

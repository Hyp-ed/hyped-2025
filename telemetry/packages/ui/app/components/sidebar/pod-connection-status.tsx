import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from '@/components/ui/tooltip';
import { usePod } from '@/context/pods';
import { cn } from '@/lib/utils';
import { POD_CONNECTION_STATUS } from '@/types/PodConnectionStatus';
import { useEffect, useState } from 'react';

/**
 * Displays the connection status of a pod to the base station (GUI).
 * @param podId The ID of the pod
 */
export const PodConnectionStatus = ({ podId }: { podId: string }) => {
	const { connectionStatus, connectionEstablished } = usePod(podId);

	const [uptime, setUptime] = useState(0);

	// Update the uptime every second
	useEffect(() => {
		if (connectionStatus !== POD_CONNECTION_STATUS.CONNECTED) {
			setUptime(0);
			return;
		}
		const interval = setInterval(() => {
			setUptime((uptime) => uptime + 1);
		}, 1000);
		return () => clearInterval(interval);
	}, [connectionStatus]);

	const CONNECTED = connectionStatus === POD_CONNECTION_STATUS.CONNECTED;
	const UNKNOWN = connectionStatus === POD_CONNECTION_STATUS.UNKNOWN;
	const DISCONNECTED = connectionStatus === POD_CONNECTION_STATUS.DISCONNECTED;
	const ERROR = connectionStatus === POD_CONNECTION_STATUS.ERROR;

	return (
		<TooltipProvider>
			<Tooltip delayDuration={300}>
				<TooltipTrigger className="text-left">
					<div className="flex gap-2 items-center">
						<div
							className={cn(
								'w-2 h-2 rounded-full',
								CONNECTED && 'bg-green-500 animate-[pulse_linear_1s_infinite]',
								UNKNOWN && 'bg-orange-500 animate-[pulse_linear_0.5s_infinite]',
								(DISCONNECTED || ERROR) && 'bg-red-500',
							)}
						/>
						<p
							className={cn(
								'text-sm italic',
								CONNECTED && 'text-green-500',
								UNKNOWN && 'text-orange-500',
								(DISCONNECTED || ERROR) && 'text-red-500',
							)}
						>
							{connectionStatus}
						</p>
					</div>
				</TooltipTrigger>
				<TooltipContent
					side="bottom"
					className="border-none bg-black shadow-2xl"
				>
					{/* <TooltipArrow width={15} height={8} className="left-0" /> */}
					<p className="text-sm">
						<span className="font-bold">Status:</span> {connectionStatus}
					</p>
					<p className="text-sm">
						<span className="font-bold">GUI connection uptime:</span>{' '}
						{/* uptime in s then x mins, y secs etc. */}
						{Math.floor(uptime / 60)} mins {uptime % 60} secs
					</p>
					<p className="text-sm">
						<span className="font-bold">GUI connection established:</span>{' '}
						{/* connection time in mm:hh:ss */}
						{connectionEstablished
							? connectionEstablished.toLocaleTimeString()
							: 'N/A'}
					</p>
				</TooltipContent>
			</Tooltip>
		</TooltipProvider>
	);
};

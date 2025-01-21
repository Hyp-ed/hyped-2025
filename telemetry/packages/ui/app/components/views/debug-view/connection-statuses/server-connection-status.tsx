import { LatencyChart } from '@/components/shared/latency-chart';
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from '@/components/ui/card';
import { config } from '@/config';
import { useLiveLogs } from '@/context/live-logs';
import type { PreviousLatenciesType } from '@/context/pods';
import { Server } from 'lucide-react';
import { useEffect, useState } from 'react';

/**
 * The number of previous latencies to show in the chart.
 */
const NUM_PREVIOUS_LATENCIES = 10 as const;

/**
 * The maximum latency before the server is considered disconnected.
 */
const SERVER_MAX_LATENCY = 100 as const;

/**
 * Displays the connection status of the telemetry server and the latency between the GUI and the server.
 * @returns The server connection status component
 */
export const ServerConnectionStatus = () => {
	const { isConnected } = useLiveLogs();

	const [previousLatencies, setPreviousLatencies] =
		useState<PreviousLatenciesType>([]);
	const [lastLatency, setLastLatency] = useState<number>();

	const connected = lastLatency ? lastLatency < SERVER_MAX_LATENCY : false;

	useEffect(function setUpLatencyCheck() {
		// Check for server ping every second and set the latency
		const interval = setInterval(() => {
			const start = Date.now();
			fetch(`${config.SERVER_ENDPOINT}/ping`)
				.then(() => {
					// The latency is the time it took to get a response from the server (round trip)
					const latency = Date.now() - start;
					setLastLatency(latency);
					setPreviousLatencies((previousLatencies) => {
						if (previousLatencies.length >= NUM_PREVIOUS_LATENCIES) {
							previousLatencies.shift();
						}
						return [...previousLatencies, { index: Date.now(), latency }];
					});
				})
				.catch(() => {
					setLastLatency(undefined);
					setPreviousLatencies([]);
				});
		}, 1000);

		// Clear interval on unmount
		return () => clearInterval(interval);
	}, []);

	return (
		<Card>
			<CardHeader>
				<CardTitle className="flex gap-2">
					<Server /> Server
				</CardTitle>
				<CardDescription>
					GUI connection to the Telemetry Server
				</CardDescription>
			</CardHeader>
			<CardContent className="space-y-2">
				<div className="flex gap-2 items-center text-sm">
					{connected ? (
						<>
							<div className="w-2 h-2 rounded-full bg-green-500 animate-[pulse_linear_1s_infinite]" />
							<p className="text-sm italic text-green-500">Connected</p>
						</>
					) : (
						<>
							<div className="w-2 h-2 rounded-full bg-red-500" />
							<p className="text-sm italic text-red-500">Disconnected</p>
						</>
					)}
				</div>
				<p className="text-sm">
					Latency: {connected ? <b>{lastLatency}ms</b> : 'N/A'}
				</p>
				<LatencyChart data={previousLatencies} maxValue={10} />
				<div className="flex gap-2 items-center text-sm">
					Live Logs:{' '}
					{isConnected ? (
						<>
							<div className="w-2 h-2 rounded-full bg-green-500 animate-[pulse_linear_1s_infinite]" />
							<p className="text-sm italic text-green-500">Connected</p>
						</>
					) : (
						<>
							<div className="w-2 h-2 rounded-full bg-red-500" />
							<p className="text-sm italic text-red-500">Disconnected</p>
						</>
					)}
				</div>
			</CardContent>
			<CardFooter className="text-sm text-muted-foreground">
				{config.SERVER_ENDPOINT}
			</CardFooter>
		</Card>
	);
};

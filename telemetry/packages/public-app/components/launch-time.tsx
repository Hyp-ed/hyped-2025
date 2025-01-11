'use client';

import { getLaunchTime } from '@/helpers';
import { Card, Metric, Text } from '@tremor/react';
import { BadgeDelta } from '@tremor/react';
import { useQuery } from 'react-query';

/**
 * The launch time card. Fetches the launch time from the server.
 * @returns The launch time card.
 */
export function LaunchTime() {
	const { data } = useQuery('launch-time', async () => await getLaunchTime(), {
		refetchInterval: 900,
	});

	// Calculate the time since launch
	const launchTime = data?.launchTime || -1;
	const currentTimeInSeconds = Date.now();
	const timeSinceLaunch =
		launchTime > 0 ? (currentTimeInSeconds - launchTime) / 1000 : -1;

	return (
		<Card decoration="top" decorationColor="red" className="space-y-2">
			<div>
				<Text>Time since launch</Text>
				<Metric>
					{timeSinceLaunch > -1
						? `${Math.floor(timeSinceLaunch / 60)}m ${Math.floor(timeSinceLaunch % 60)}s`
						: 'Not launched yet'}
				</Metric>
			</div>
			<div>
				<BadgeDelta
					size="lg"
					deltaType={launchTime > 0 ? 'moderateIncrease' : 'moderateDecrease'}
				>
					{launchTime > 0 ? 'LIVE' : 'Not Launched'}
				</BadgeDelta>
			</div>
		</Card>
	);
}

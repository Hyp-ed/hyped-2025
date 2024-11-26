'use client';

import { getVelocity } from '@/helpers';
import type { HistoricalValueResponse } from '@hyped/telemetry-types';
import { Card, LineChart, Metric, Text } from '@tremor/react';
import { format } from 'date-fns';
import { useQuery } from 'react-query';

/**
 * The velocity chart. Fetches velocity data from the server.
 * @returns The velocity chart.
 */
export const VelocityGraph = () => {
	const { data, isLoading, error } = useQuery(
		'velocity',
		async () => await getVelocity(),
		{
			refetchInterval: 1000,
		},
	);

	if (isLoading)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Velocity</Metric>
				<Text>Loading...</Text>
			</Card>
		);
	if (error)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Velocity</Metric>
				<Text>Error fetching velocity data</Text>
			</Card>
		);

	const velocityData = formatData(data);

	return (
		<Card decoration="top" decorationColor="red">
			<Metric>Velocity</Metric>
			<LineChart
				className="mt-6"
				data={velocityData}
				index="time"
				categories={['velocity']}
				colors={['red']}
				yAxisWidth={30}
			/>
		</Card>
	);
};

/**
 * Formats the data to be displayed on the chart.
 * @param data The data to format.
 * @returns The formatted data.
 */
const formatData = (data: HistoricalValueResponse | undefined) =>
	data
		? data.map((d) => {
				const time = new Date(d.timestamp);
				return {
					time: format(time, 'HH:mm:ss'),
					velocity: d.value,
				};
			})
		: [];

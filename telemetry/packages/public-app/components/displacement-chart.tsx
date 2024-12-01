'use client';

import { getDisplacement } from '@/helpers';
import type { HistoricalValueResponse } from '@hyped/telemetry-types';
import { Card, LineChart, Metric, Text } from '@tremor/react';
import format from 'date-fns/format';
import { useQuery } from 'react-query';

/**
 * The displacement chart. Fetches displacement data from the server.
 * @returns The displacement chart.
 */
export const DisplacementChart = () => {
	const { data, isLoading, error } = useQuery(
		'displacement',
		async () => await getDisplacement(),
		{
			refetchInterval: 1000,
		},
	);

	if (isLoading)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Displacement</Metric>
				<Text>Loading...</Text>
			</Card>
		);
	if (error)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Displacement</Metric>
				<Text>Error fetching displacement data</Text>
			</Card>
		);

	const displacementData = formatData(data);

	return (
		<Card decoration="top" decorationColor="red">
			<Metric>Displacement</Metric>
			<LineChart
				className="mt-6"
				data={displacementData}
				index="time"
				categories={['displacement']}
				colors={['red']}
				yAxisWidth={40}
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
					displacement: d.value,
				};
			})
		: [];

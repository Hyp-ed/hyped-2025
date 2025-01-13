'use client';

import { getLevitationHeight } from '@/helpers';
import type { LevitationHeightResponse } from '@hyped/telemetry-types';
import {
	BadgeDelta,
	Button,
	Card,
	Flex,
	Metric,
	ProgressBar,
	Text,
	Title,
} from '@tremor/react';
import { useState } from 'react';
import { useQuery } from 'react-query';

export default function LevitationHeight() {
	const [showMore, setShowMore] = useState(false);

	const { data, isLoading, error } = useQuery<LevitationHeightResponse>(
		'levitation-height',
		async () => await getLevitationHeight(),
		{
			refetchInterval: 1000,
		},
	);

	if (isLoading || !data)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Levitation Height</Metric>
				<Text>Loading...</Text>
			</Card>
		);
	if (error)
		return (
			<Card decoration="top" decorationColor="red">
				<Metric>Levitation Height</Metric>
				<Text>Error fetching levitation height data</Text>
			</Card>
		);

	/**
	 * Checks if the pod is levitated based on the levitation height data.
	 * If any of the levitation height values are greater than 0, the pod is considered levitated.
	 */
	const isLevitated = data
		? Object.keys(data).some((key) => data[key]?.value > 0)
		: false;

	const keys = Object.keys(data);
	const numSensors = keys.length;

	return (
		<Card decoration="top" decorationColor="red">
			<Flex alignItems="start">
				<Title>Levitation Height</Title>
				{data ? (
					<BadgeDelta
						deltaType={isLevitated ? 'moderateIncrease' : 'moderateDecrease'}
					>
						Elevated
					</BadgeDelta>
				) : null}
			</Flex>
			<Metric>Pod Height</Metric>
			{data ? (
				keys
					// unless showMore is true, only show the first 4 (if possible) or half of the data
					.slice(0, showMore ? numSensors : numSensors > 4 ? 4 : numSensors / 2)
					.map((levitationHeight) => {
						const { id, value } = data[levitationHeight];
						return (
							<div key={id} className="space-y-2 mt-4">
								<Flex>
									<Text>{id}</Text>
									<Text>{`${value} mm`}</Text>
								</Flex>
								<ProgressBar value={value} />
							</div>
						);
					})
			) : (
				<div>no data </div>
			)}
			<Flex className="mt-6 pt-4 border-t">
				<Button
					size="xs"
					variant="light"
					iconPosition="right"
					onClick={() => setShowMore(!showMore)}
				>
					{showMore ? 'Show Less' : 'Show More'}
				</Button>
			</Flex>
		</Card>
	);
}

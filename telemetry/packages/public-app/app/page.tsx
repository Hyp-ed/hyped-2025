'use client';

import LoadingScreen from '@/app/loading-screen';
import { useEffect, useState } from 'react';
import Cards from './cards';
import { STATUSES, type Status } from './loading-screen';

export default function Home() {
	const [status, setStatus] = useState<Status>(STATUSES.AUTHENTICATING);

	// Simulate a loading process
	// Not the best way to do this but it'll do
	useEffect(() => {
		setTimeout(() => {
			setStatus(STATUSES.PROCESSING);
		}, 1500);
		setTimeout(() => {
			setStatus(STATUSES.GRANTING_ACCESS);
		}, 3000);
		setTimeout(() => {
			setStatus(STATUSES.DONE);
		}, 4500);
	}, []);

	return (
		<main className="dark:bg-black bg-white px-4 py-12">
			{status !== STATUSES.DONE ? <LoadingScreen status={status} /> : <Cards />}
		</main>
	);
}

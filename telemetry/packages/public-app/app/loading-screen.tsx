import React from 'react';
import '../app/globals.css';
import Image from 'next/image';

/**
 * Loading screen statuses. Used to simulate a loading process.
 */
export const STATUSES = {
	AUTHENTICATING: 'authenticating',
	PROCESSING: 'processing',
	GRANTING_ACCESS: 'granting-access',
	DONE: 'done',
};
export type Status = (typeof STATUSES)[keyof typeof STATUSES];

export default function LoadingScreen({ status }: { status: Status }) {
	const text: Record<Status, string> = {
		[STATUSES.AUTHENTICATING]: 'Authenticating...',
		[STATUSES.PROCESSING]: 'Performing Security Checks...',
		[STATUSES.GRANTING_ACCESS]: 'Granting access...',
	};

	return (
		<div className="fade-in-image bg-black text-white text-lg justify-center content-center items-center flex flex-col h-[90vh] overflow-y-hidden gap-4">
			<Image alt="hyped logo" src="/hyped.svg" width={100} height={100} />
			<div className="processing">
				<h1>{text[status]}</h1>
			</div>
		</div>
	);
}

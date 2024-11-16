import { config } from '@/config';
import { log } from '@/lib/logger';
import type { PodId } from '@hyped/telemetry-constants';
import { createContext, useContext, useState } from 'react';

export const ERROR_IDS = {
	POD_DISCONNECT: 'POD_DISCONNECT',
	POD_FAILURE_STATE: 'POD_FAILURE_STATE',
	TEST: 'TEST',
};

type ErrorIds = (typeof ERROR_IDS)[keyof typeof ERROR_IDS];

type ErrorMessage = {
	id: ErrorIds;
	title: string;
	message: string;
	podId?: PodId;
	timestamp: number;
	acknowledge: () => void;
};

type ErrorContextType = {
	errors: ErrorMessage[];
	raiseError: (
		id: ErrorIds,
		title: string,
		message: string,
		podId?: PodId,
	) => void;
};

const ErrorContext = createContext<ErrorContextType | null>(null);

interface ErrorProviderProps {
	children: React.ReactNode;
}

/**
 * Error Context Provider.
 * Provides a context for error messages, including functions to acknowledge them, and a function to raise new errors.
 */
export const ErrorProvider = ({ children }: ErrorProviderProps) => {
	const [errors, setErrors] = useState<ErrorMessage[]>([]);

	const raiseError = (
		id: ErrorIds,
		title: string,
		message: string,
		podId?: PodId,
	) => {
		// If the error is a pod disconnect error and the config is set to disable it, don't raise the error
		if (
			config.DISABLE_POD_DISCONNECTED_ERROR &&
			id === ERROR_IDS.POD_DISCONNECT
		) {
			return;
		}

		const error: ErrorMessage = {
			id,
			title,
			message,
			podId,
			timestamp: Date.now(),
			acknowledge: () => {
				// Remove the error from the list of errors (by id and podId)
				setErrors((errors) =>
					errors.filter((e) => !(id == e.id && podId == e.podId)),
				);
				log(`Acknowledged error dialog for: ${title}`);
			},
		};

		log(`Critical error occurred. ${title}: ${message}`);

		// Add the error to the list of errors, sorted by timestamp
		setErrors((errors) =>
			[...errors, error]
				// remove duplicates
				.filter(
					(e, i, a) =>
						a.findIndex((ee) => ee.id === e.id && ee.podId === e.podId) === i,
				)
				// sort by most recent
				.sort((a, b) => b.timestamp - a.timestamp),
		);
	};

	return (
		<ErrorContext.Provider value={{ errors, raiseError }}>
			{children}
		</ErrorContext.Provider>
	);
};

export const useErrors = () => {
	const context = useContext(ErrorContext);
	if (!context) {
		throw new Error('useErrors must be used within ErrorContextProvider');
	}
	return context;
};

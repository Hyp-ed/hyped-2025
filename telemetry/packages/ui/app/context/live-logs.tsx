import { config } from '@/config';
import { createContext, useContext, useEffect, useState } from 'react';
import { io } from 'socket.io-client';

// Socket.io client for live logs from the telemetry server
const socket = io(config.SERVER_ENDPOINT, {
	path: '/live-logs',
});

/**
 * Log levels for the live logs. These are the same as the Winston log levels.
 */
export const LOG_LEVELS = {
	INFO: 'info',
	WARN: 'warn',
	ERROR: 'error',
	DEBUG: 'debug',
	VERBOSE: 'verbose',
} as const;

export type LogLevel = (typeof LOG_LEVELS)[keyof typeof LOG_LEVELS];

export type Log = {
	context: string;
	stack?: any;
	level: LogLevel;
	message: string;
	timestamp: string;
};

type LiveLogsContext = {
	isConnected: boolean;
	logs: Log[];
	clearAll: () => void;
};

const LiveLogsContext = createContext<LiveLogsContext | null>(null);

/**
 * Provider for the live logs context.
 */
export const LiveLogsProvider = ({
	children,
}: {
	children: React.ReactNode;
}) => {
	const [isConnected, setIsConnected] = useState(false);
	const [logs, setLogs] = useState<Log[]>([]);

	useEffect(() => {
		function onConnect() {
			setIsConnected(true);
		}

		function onDisconnect() {
			setIsConnected(false);
		}

		// When we receive a new log from the server, add it to the logs array
		function onLog(log: Log) {
			// Only keep the last 100 logs
			if (logs.length > 100) {
				setLogs((logs) => logs.slice(1));
			}
			setLogs((logs) => [...logs, log]);
		}

		socket.on('connect', onConnect);
		socket.on('disconnect', onDisconnect);
		socket.on('log', onLog);

		return () => {
			socket.off('connect', onConnect);
			socket.off('disconnect', onDisconnect);
			socket.off('log', onLog);
		};
	}, [logs.length]);

	/**
	 * Clears all logs.
	 */
	function clearAll() {
		setLogs([]);
	}

	return (
		<LiveLogsContext.Provider
			value={{
				isConnected,
				logs,
				clearAll,
			}}
		>
			{children}
		</LiveLogsContext.Provider>
	);
};

/**
 * Hook to use the live logs context.
 * Throws an error if used outside of the LiveLogsProvider.
 */
export const useLiveLogs = () => {
	const context = useContext(LiveLogsContext);
	if (!context) {
		throw new Error('useLiveLogs must be used within LiveLogsProvider');
	}
	return context;
};

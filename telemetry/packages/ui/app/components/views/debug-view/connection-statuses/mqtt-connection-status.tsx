import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from '@/components/ui/card';
import { useMQTT } from '@/context/mqtt';
import type { MQTTConnectionStatusType } from '@/types/MQTTConnectionStatus';
import { Radio } from 'lucide-react';

/**
 * Displays the connection status of the base station (GUI) to the MQTT broker.
 */
export const MqttConnectionStatus = () => {
	const { mqttConnectionStatus, connectedAt, broker } = useMQTT();

	// Maps the connection statuses to React components
	const statusComponentMap: Record<MQTTConnectionStatusType, React.ReactNode> =
		{
			CONNECTING: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-yellow-500 animate-[pulse_linear_0.5s_infinite]" />
					<p className="text-sm italic text-yellow-500">Connecting</p>
				</div>
			),
			CONNECTED: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-green-500 animate-[pulse_linear_1s_infinite]" />
					<p className="text-sm italic text-green-500">Connected</p>
				</div>
			),
			DISCONNECTED: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-red-500" />
					<p className="text-sm italic text-red-500">Disconnected</p>
				</div>
			),
			UNKNOWN: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-white animate-[pulse_linear_2s_infinite]" />
					<p className="text-sm italic text-white">Unknown</p>
				</div>
			),
			ERROR: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-red-500 animate-[pulse_linear_0.2s_infinite]" />
					<p className="text-sm italic text-red-500">Error</p>
				</div>
			),
			RECONNECTING: (
				<div className="flex gap-2 items-center">
					<div className="w-2 h-2 rounded-full bg-yellow-500 animate-[pulse_linear_0.5s_infinite]" />
					<p className="text-sm italic text-yellow-500">Reconnecting</p>
				</div>
			),
		};

	return (
		<Card>
			<CardHeader>
				<CardTitle className="flex gap-2">
					<Radio /> MQTT
				</CardTitle>
				<CardDescription>GUI connection to the MQTT broker</CardDescription>
			</CardHeader>
			<CardContent>
				<div className="flex flex-col gap-2">
					{statusComponentMap[mqttConnectionStatus]}
					{connectedAt ? (
						<p className="text-sm">
							Connected at: <b>{new Date(connectedAt).toLocaleTimeString()}</b>
						</p>
					) : (
						false
					)}
				</div>
			</CardContent>
			<CardFooter className="text-sm text-muted-foreground">
				{broker}
			</CardFooter>
		</Card>
	);
};

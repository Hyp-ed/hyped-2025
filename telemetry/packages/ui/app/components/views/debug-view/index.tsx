import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
} from '@/components/ui/resizable';
import { config } from '@/config';
import { useCurrentPod } from '@/context/pods';
import { ConnectionStatuses } from './connection-statuses/connection-statuses';
import { FullControls } from './full-controls';
import { MqttSender } from './mqtt-sender';
import { PodStateUpdater } from './pod-state-updater';

/**
 * Debug view. Contains components for debugging.
 * Includes:
 * - Full set of controls for pod
 * - Details connection statuses and latencies (connection to MQTT broker, connection to pod, etc.)
 * - Custom MQTT message sender
 * @returns The debug view
 */
export const DebugView = () => {
	const { currentPod: podId } = useCurrentPod();

	const showExternalDebuggingTools = config.EXTENDED_DEBUGGING_TOOLS ?? false;

	return (
		<ResizablePanelGroup direction="vertical">
			<ResizablePanel defaultSize={20}>
				<ResizablePanelGroup direction="horizontal" className="w-full h-full">
					<ResizablePanel defaultSize={50} className="flex items-center">
						<div className="flex justify-center flex-col items-center w-full gap-2 p-2">
							<h1 className="font-bold text-xl text-foreground">Placeholder</h1>
							<p className="text-muted-foreground text-center">
								Something useful will go here at some point...
							</p>
						</div>
					</ResizablePanel>
					<ResizableHandle withHandle />
					<ResizablePanel defaultSize={50} className="flex items-center gap-2">
						{showExternalDebuggingTools ? (
							<PodStateUpdater podId={podId} />
						) : (
							<p className="mx-auto p-2">
								Enable EXTENDED_DEBUGGING_TOOLS in .env to see more debugging
								tools.
							</p>
						)}
					</ResizablePanel>
				</ResizablePanelGroup>
			</ResizablePanel>
			<ResizableHandle withHandle />
			<ResizablePanel defaultSize={30}>
				<FullControls podId={podId} />
			</ResizablePanel>
			<ResizableHandle withHandle />
			<ResizablePanel defaultSize={50}>
				<ResizablePanelGroup direction="horizontal" className="w-full h-full">
					<ResizablePanel
						defaultSize={60}
						className="flex items-center justify-center"
					>
						<ConnectionStatuses />
					</ResizablePanel>
					<ResizableHandle withHandle />
					<ResizablePanel
						defaultSize={40}
						className="flex items-center justify-center"
					>
						<MqttSender />
					</ResizablePanel>
				</ResizablePanelGroup>
			</ResizablePanel>
		</ResizablePanelGroup>
	);
};

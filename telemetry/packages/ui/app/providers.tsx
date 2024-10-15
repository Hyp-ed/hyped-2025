import { LiveLogsProvider } from './context/live-logs';
import { MQTTProvider } from './context/mqtt';
import { PodsProvider } from './context/pods';
import { config } from './config';
import { QoS } from './types/mqtt';
import { ErrorProvider } from './context/errors';

/**
 * Provider for all the contexts.
 * @param children The children to render
 */
export const Providers = ({ children }: { children: React.ReactNode }) => (
  <ErrorProvider>
    <MQTTProvider broker={config.MQTT_BROKER} qos={config.MQTT_QOS as QoS}>
      <PodsProvider>
        <LiveLogsProvider>{children}</LiveLogsProvider>
      </PodsProvider>
    </MQTTProvider>
  </ErrorProvider>
);

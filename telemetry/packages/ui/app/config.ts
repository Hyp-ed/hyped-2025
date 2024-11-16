// Here we validate the environment variables and cast them to the correct types using zod

import { z } from 'zod';

const booleanFromString = z
  .enum(['true', 'false'])
  .transform((v) => v === 'true');

const envSchema = z.object({
  VITE_SERVER_ENDPOINT: z.string().url(),
  VITE_MQTT_BROKER: z.string(),
  VITE_MQTT_QOS: z.coerce.number().refine(
    (n) => {
      return n === 0 || n === 1 || n === 2;
    },
    {
      message: 'QoS must be 0, 1, or 2',
    },
  ),
  VITE_DISABLE_POD_DISCONNECTED_ERROR: booleanFromString.optional(),
  VITE_EXTENDED_DEBUGGING_TOOLS: booleanFromString.optional(),
});

const result = envSchema.safeParse({
  VITE_SERVER_ENDPOINT: import.meta.env.VITE_SERVER_ENDPOINT,
  VITE_MQTT_BROKER: import.meta.env.VITE_MQTT_BROKER,
  VITE_MQTT_QOS: import.meta.env.VITE_MQTT_QOS,
  VITE_DISABLE_POD_DISCONNECTED_ERROR: import.meta.env
    .VITE_DISABLE_POD_DISCONNECTED_ERROR,
  VITE_EXTENDED_DEBUGGING_TOOLS: import.meta.env.VITE_EXTENDED_DEBUGGING_TOOLS,
});

if (!result.success) {
  throw new Error(
    'Missing or invalid environment variables!' + JSON.stringify(result.error),
  );
}

// Remove the `VITE_` prefix from the object keys and cast the result to the correct type
const config = Object.fromEntries(
  Object.entries(result.data).map(([key, value]) => [
    key.replace('VITE_', ''),
    value,
  ]),
) as {
  [K in keyof typeof result.data as K extends `VITE_${infer P}`
    ? P
    : never]: (typeof result.data)[K];
};

export { config };

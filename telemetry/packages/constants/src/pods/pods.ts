import * as fs from 'node:fs';
import * as path from 'node:path';
import { PodSchema } from '@hyped/telemetry-types';
import * as YAML from 'yaml';
import { z } from 'zod';

const CONFIG_FILE_NAME = 'pods.yaml';
// Root of hyped repo
const CONFIG_PATH = path.join(
	__dirname,
	'..',
	'..',
	'..',
	'..',
	'..',
	'config',
	CONFIG_FILE_NAME,
);

const RawPodsSchema = z.object({
	pods: z.record(
		z.object({
			label: z.string(),
			measurements: z.record(z.object({}).passthrough()),
			statuses: z.record(z.object({}).passthrough()),
		}),
	),
});

const yamlContent = fs.readFileSync(CONFIG_PATH, 'utf8');
const yamlData = RawPodsSchema.parse(YAML.parse(yamlContent));

export const pods = Object.fromEntries(
	Object.entries(yamlData.pods).map(([podId, podData]) => [
		podId,
		PodSchema.parse({
			id: podId,
			...podData,
			measurements: Object.fromEntries(
				Object.entries(podData.measurements).map(([id, measurement]) => [
					id,
					{ id, ...measurement },
				]),
			),
			statuses: Object.fromEntries(
				Object.entries(podData.statuses).map(([id, status]) => [
					id,
					{ id, ...status },
				]),
			),
		}),
	]),
);

export const podIds = Object.keys(pods);

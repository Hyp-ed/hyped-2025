import * as fs from "node:fs";
import * as path from "node:path";
import { PodSchema } from "@hyped/telemetry-types";
import * as YAML from "yaml";
import { z } from "zod";
import { measurementTypes } from "./types";

const CONFIG_FILE_NAME = "pods.yaml";
// Root of hyped repo
const CONFIG_PATH = path.join(
	__dirname,
	"..",
	"..",
	"..",
	"..",
	"..",
	"config",
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

// We also want to check the 'type' field of each measurement and status is one of the object types
// It would be unwise to add this directly to the schema (in the types package) because it would
// break the circular dependency between the types and constants packages.
type MeasurementType = (typeof measurementTypes)[number];
const validateType = (
	ctx: z.RefinementCtx,
	items: Record<string, unknown>,
	type: "measurements" | "statuses",
) => {
	for (const [id, item] of Object.entries(items)) {
		const itemType = (item as { type: string }).type;
		if (!measurementTypes.includes(itemType as MeasurementType)) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				message: `Invalid ${type.slice(0, -1)} type "${itemType}"`,
				path: [type, id, "type"],
			});
		}
	}
};
const ExtendedPodSchema = PodSchema.superRefine((pod, ctx) => {
	validateType(ctx, pod.measurements, "measurements");
	validateType(ctx, pod.statuses, "statuses");
});

const yamlContent = fs.readFileSync(CONFIG_PATH, "utf8");
const yamlData = RawPodsSchema.parse(YAML.parse(yamlContent));

export const pods = Object.fromEntries(
	Object.entries(yamlData.pods).map(([podId, podData]) => [
		podId,
		ExtendedPodSchema.parse({
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

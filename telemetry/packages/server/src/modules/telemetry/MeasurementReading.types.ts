import { zodEnumFromObjKeys } from "@/modules/common/utils/zodEnumFromObjKeys";
import { pods } from "@hyped/telemetry-constants";
import { z } from "zod";

export const MeasurementReadingSchema = z.object({
	podId: zodEnumFromObjKeys(pods),
	measurementKey: z.string(),
	timestamp: z.string(), // to handle nanoseconds timestamp
	value: z.number(),
});

export type MeasurementReading = z.infer<typeof MeasurementReadingSchema>;

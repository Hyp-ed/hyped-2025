import { zodEnumFromObjKeys } from "@/modules/common/utils/zodEnumFromObjKeys";
import { pods } from "@hyped/telemetry-constants";
import { z } from "zod";

export const StatusEntrySchema = z.object({
	podId: zodEnumFromObjKeys(pods),
	statusId: z.string(),
	timestamp: z.string(),
	value: z.number(),
});

export type StatusEntry = z.infer<typeof StatusEntrySchema>;

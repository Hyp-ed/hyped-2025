import { z } from 'zod';

export const LimitSchema = z.object({
	low: z.number(),
	high: z.number(),
});

export type Limit = z.infer<typeof LimitSchema>;

export const MeasurementSchema = z.object({
	key: z.string(),
	name: z.string(),
	unit: z.string(),
	format: z.string(),
	limits: z.object({
		critical: LimitSchema,
		warning: LimitSchema.optional()
	}),
});

export type Measurement = z.infer<typeof MeasurementSchema>;

export const PodSchema = z.object({
	id: z.string(),
	name: z.string(),
	measurements: z.record(
		z.string(),
		MeasurementSchema
	),
});

export type Pod = z.infer<typeof PodSchema>;






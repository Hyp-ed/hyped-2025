import { z } from 'zod';

export const LimitSchema = z.object({
	low: z.number(),
	high: z.number(),
});

export type Limit = z.infer<typeof LimitSchema>;

export const MeasurementLimitsSchema = z.object({
	critical: LimitSchema,
	warning: LimitSchema.optional(),
});

export type MeasurementLimits = z.infer<typeof MeasurementLimitsSchema>;

export const MeasurementSchema = z.object({
	id: z.string(),
	label: z.string(),
	unit: z.string(),
	format: z.string(),
	limits: MeasurementLimitsSchema,
});

export type Measurement = z.infer<typeof MeasurementSchema>;

export const StatusSchema = z.object({
	id: z.string(),
	label: z.string(),
	format: z.string(),
	values: z.array(
		z.object({
			value: z.number(),
			label: z.string(),
		}),
	),
});

export type Status = z.infer<typeof StatusSchema>;

export const PodSchema = z.object({
	id: z.string(),
	label: z.string(),
	mode: z.enum(['ALL_SYSTEMS_ON', 'LEVITATION_ONLY', 'LIM_ONLY']),
	measurements: z.record(z.string(), MeasurementSchema),
	statuses: z.record(z.string(), StatusSchema),
});

export type Pod = z.infer<typeof PodSchema>;

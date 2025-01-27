import { z } from 'zod';

export const PodSchema = z.object({
	id: z.string(),
	name: z.string(),
	measurements: z.record(
		z.string(),
		z.object({
			name: z.string(),
			unit: z.string(),
			format: z.string(),
			limits: z.object({
				critical: z.object({
					min: z.number(),
					max: z.number(),
				}),
				warning: z.object({
					min: z.number(),
					max: z.number(),
				}),
			}),
		}),
	),
});

import { pods } from '@hyped/telemetry-constants';
import { zodEnumFromObjKeys } from '@/modules/common/utils/zodEnumFromObjKeys';
import { z } from 'zod';

export const MeasurementReadingSchema = z
  .object({
    podId: zodEnumFromObjKeys(pods),
    measurementKey: z.string(),
    timestamp: z.string(), // to handle nanoseconds timestamp
    value: z.number(),
  })
  // Validate measurement exists and enum value is valid (if applicable)
  .refine(
    ({ podId, measurementKey, value }) => {
      const measurement = pods[podId]['measurements'][measurementKey];

      if (!measurement) {
        return false;
      }

      // Validate enum values
      if (measurement.format === 'enum') {
        const enumValue = measurement.enumerations.find(
          (e) => e.value === value,
        );

        if (!enumValue) {
          return false;
        }
      }

      // Validate integers and floats
      if (
        (measurement.format === 'float' && isNaN(value)) ||
        (measurement.format === 'integer' && !Number.isInteger(value))
      ) {
        return false;
      }

      return true;
    },
    {
      message:
        'Invalid measurement reading - measurement does not exist or invalid enum value',
    },
  );

export type MeasurementReading = z.infer<typeof MeasurementReadingSchema>;

import { OpenMctMeasurement } from '@hyped/telemetry-types';
import { OpenMCT } from 'openmct/dist/openmct';
import { Datum } from 'openmct/types/Datum';
import { Unpacked } from 'openmct/types/Unpacked';
import { AugmentedDomainObject } from '../types/AugmentedDomainObject';

/**
 * Base limits for telemetry data.
 */
const LIMITS = {
  criticalHigh: {
    cssClass: 'is-limit--upr is-limit--red',
    high: Number.POSITIVE_INFINITY,
    name: 'Critical High',
  },
  criticalLow: {
    cssClass: 'is-limit--lwr is-limit--red',
    low: Number.NEGATIVE_INFINITY,
    name: 'Critical Low',
  },
  warningHigh: {
    cssClass: 'is-limit--upr is-limit--yellow',
    name: 'Warning High',
  },
  warningLow: {
    cssClass: 'is-limit--lwr is-limit--yellow',
    name: 'Warning Low',
  },
};

/**
 * The Limit plugin for Open MCT.
 * Provides limit evaluation and retrieval for telemetry data in order to display
 * limit indicators/lines on telemetry plots.
 * @see https://github.com/nasa/openmct/blob/master/API.md#limit-evaluators-draft
 * @see https://github.com/nasa/openmct/blob/master/example/generator/SinewaveLimitProvider.js (example implementation)
 * @returns The limit plugin function.
 */
export function LimitPlugin() {
  return function install(openmct: OpenMCT) {
    const provider = {
      // We only support limits for domain objects that have limits
      supportsLimits: function (domainObject: AugmentedDomainObject) {
        return domainObject.limits !== undefined;
      },
      /**
       * Gets a limit evaluator function for a domain object which is used to
       * evaluate telemetry data against the domain object's limits.
       * @returns The limit evaluator function.
       */
      getLimitEvaluator: function () {
        return {
          evaluate: function (
            datum: Datum,
            valueMetadata: Unpacked<OpenMctMeasurement['values']>,
          ) {
            const { value } = datum;
            const { limits } = valueMetadata;

            if (!limits) {
              throw new Error('No limits found');
            }

            // Return the limit that the value exceeds

            if (value > limits.critical.high) {
              return {
                ...LIMITS.criticalHigh,
                low: limits.critical.high,
              };
            }

            if (value < limits.critical.low) {
              return {
                ...LIMITS.criticalLow,
                high: limits.critical.low,
              };
            }

            if (limits.warning) {
              if (value > limits.warning.high) {
                return {
                  ...LIMITS.warningHigh,
                  low: limits.warning.high,
                  high: limits.critical.high,
                };
              }

              if (value < limits.warning.low) {
                return {
                  ...LIMITS.warningLow,
                  high: limits.warning.low,
                  low: limits.critical.low,
                };
              }
            }
          },
        };
      },
      /**
       * Get the limits for a domain object.
       * @param domainObject The domain object to get the limits for.
       * @returns The limits for the domain object.
       */
      getLimits: function (domainObject: AugmentedDomainObject) {
        const { limits } = domainObject;
        if (!limits) {
          throw new Error('No limits found');
        }

        return {
          limits: function () {
            return Promise.resolve({
              ...(limits.warning
                ? {
                    WARNING: {
                      low: {
                        color: 'yellow',
                        value: limits.warning.low,
                      },
                      high: {
                        color: 'yellow',
                        value: limits.warning.high,
                      },
                    },
                  }
                : {}),
              CRITICAL: {
                low: {
                  color: 'red',
                  value: limits.critical.low,
                },
                high: {
                  color: 'red',
                  value: limits.critical.high,
                },
              },
            });
          },
        };
      },
    };

    openmct.telemetry.addProvider(provider);
  };
}

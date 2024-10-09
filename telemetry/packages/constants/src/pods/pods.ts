import { Pod } from '@hyped/telemetry-types';
import {
  accelerometerCommon,
  hallEffectCommon,
  keyenceCommon,
  pressureCommon,
  thermistorCommon,
  levitationHeightCommon,
} from './common';

export const POD_IDS = ['pod_1', 'pod_2024'] as const;
export type PodId = (typeof POD_IDS)[number];
export type Pods = Record<PodId, Pod>;

export const pods: Pods = {
  pod_1: {
    id: 'pod_1',
    name: 'Pod Ness',
    operationMode: 'ALL_SYSTEMS_ON',
    measurements: {
      // ************************************ ACCELEROMETERS ************************************ //
      accelerometer_1: {
        name: 'Accelerometer 1',
        key: 'accelerometer_1',
        ...accelerometerCommon,
      },
      accelerometer_2: {
        name: 'Accelerometer 2',
        key: 'accelerometer_2',
        ...accelerometerCommon,
      },
      accelerometer_3: {
        name: 'Accelerometer 3',
        key: 'accelerometer_3',
        ...accelerometerCommon,
      },
      accelerometer_4: {
        name: 'Accelerometer 4',
        key: 'accelerometer_4',
        ...accelerometerCommon,
      },
      accelerometer_avg: {
        name: 'Accelerometer Average',
        key: 'accelerometer_avg',
        ...accelerometerCommon,
      },

      // ************************************ NAVIGATION ************************************ //
      displacement: {
        name: 'Displacement',
        key: 'displacement',
        format: 'float',
        type: 'motion',
        unit: 'm',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 0,
        sampling_time: accelerometerCommon.sampling_time,
      },
      velocity: {
        name: 'Velocity',
        key: 'velocity',
        format: 'float',
        type: 'motion',
        unit: 'm/s',
        limits: {
          critical: {
            low: 0,
            high: 50,
          },
        },
        rms_noise: accelerometerCommon.rms_noise,
        sampling_time: accelerometerCommon.sampling_time,
      },
      acceleration: {
        name: 'Acceleration',
        key: 'acceleration',
        format: 'float',
        type: 'motion',
        unit: 'm/s²',
        limits: {
          critical: {
            low: 0,
            high: 5,
          },
        },
        rms_noise: accelerometerCommon.rms_noise,
        sampling_time: accelerometerCommon.sampling_time,
      },

      // ************************************ PRESSURE ************************************ //
      pressure_back_pull: {
        name: 'Pressure – Back Pull',
        key: 'pressure_back_pull',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_front_pull: {
        name: 'Pressure – Front Pull',
        key: 'pressure_front_pull',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_front_push: {
        name: 'Pressure – Front Push',
        key: 'pressure_front_push',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_back_push: {
        name: 'Pressure – Back Push',
        key: 'pressure_back_push',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_brakes_reservoir: {
        name: 'Pressure – Brakes Reservoir',
        key: 'pressure_brakes_reservoir',
        ...pressureCommon,
        limits: {
          critical: {
            low: 3,
            high: 7.4,
          },
          warning: {
            low: 3.5,
            high: 6.9,
          },
        },
      },
      pressure_active_suspension_reservoir: {
        name: 'Pressure – Active Suspension Reservoir',
        key: 'pressure_active_suspension_reservoir',
        ...pressureCommon,
        limits: {
          critical: {
            low: 3,
            high: 7.4,
          },
          warning: {
            low: 3.5,
            high: 6.9,
          },
        },
      },
      pressure_front_brake: {
        name: 'Pressure – Front Brake',
        key: 'pressure_front_brake',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 4.2,
          },
          warning: {
            low: -0.19,
            high: 4,
          },
        },
      },
      pressure_back_brake: {
        name: 'Pressure – Back Brake',
        key: 'pressure_back_brake',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 4.2,
          },
          warning: {
            low: -0.19,
            high: 4,
          },
        },
      },

      // ************************************ THERMISTORS ************************************ //
      thermistor_1: {
        name: 'Thermistor 1',
        key: 'thermistor_1',
        ...thermistorCommon,
      },
      thermistor_2: {
        name: 'Thermistor 2',
        key: 'thermistor_2',
        ...thermistorCommon,
      },
      thermistor_3: {
        name: 'Thermistor 3',
        key: 'thermistor_3',
        ...thermistorCommon,
      },
      thermistor_4: {
        name: 'Thermistor 4',
        key: 'thermistor_4',
        ...thermistorCommon,
      },
      thermistor_5: {
        name: 'Thermistor 5',
        key: 'thermistor_5',
        ...thermistorCommon,
      },
      thermistor_6: {
        name: 'Thermistor 6',
        key: 'thermistor_6',
        ...thermistorCommon,
      },
      thermistor_7: {
        name: 'Thermistor 7',
        key: 'thermistor_7',
        ...thermistorCommon,
      },
      thermistor_8: {
        name: 'Thermistor 8',
        key: 'thermistor_8',
        ...thermistorCommon,
      },
      thermistor_9: {
        name: 'Thermistor 9',
        key: 'thermistor_9',
        ...thermistorCommon,
      },
      thermistor_10: {
        name: 'Thermistor 10',
        key: 'thermistor_10',
        ...thermistorCommon,
      },
      thermistor_11: {
        name: 'Thermistor 11',
        key: 'thermistor_11',
        ...thermistorCommon,
      },
      thermistor_12: {
        name: 'Thermistor 12',
        key: 'thermistor_12',
        ...thermistorCommon,
      },
      // thermistor_13: {
      //   name: 'Thermistor 13',
      //   key: 'thermistor_13',
      //   ...thermistorCommon,
      // },
      // thermistor_14: {
      //   name: 'Thermistor 14',
      //   key: 'thermistor_14',
      //   ...thermistorCommon,
      // },
      // thermistor_15: {
      //   name: 'Thermistor 15',
      //   key: 'thermistor_15',
      //   ...thermistorCommon,
      // },
      // thermistor_16: {
      //   name: 'Thermistor 16',
      //   key: 'thermistor_16',
      //   ...thermistorCommon,
      // },

      // ************************************ HALL EFFECTS ************************************ //
      hall_effect_1: {
        name: 'Hall Effect 1',
        key: 'hall_effect_1',
        ...hallEffectCommon,
      },
      hall_effect_2: {
        name: 'Hall Effect 2',
        key: 'hall_effect_2',
        ...hallEffectCommon,
      },

      // ************************************ STATUS ************************************ //
      brake_clamp_status: {
        name: 'Brake Clamp Status',
        key: 'brake_clamp_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'CLAMPED',
          },
          {
            value: 0,
            string: 'UNCLAMPED',
          },
        ],
      },
      pod_raised_status: {
        name: 'Pod Raised Status',
        key: 'pod_raised_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'RAISED',
          },
          {
            value: 0,
            string: 'LOWERED',
          },
        ],
      },

      battery_status: {
        name: 'Battery Status',
        key: 'battery_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'HEALTHY',
          },
          {
            value: 0,
            string: 'UNHEALTHY',
          },
        ],
      },

      motor_controller_status: {
        name: 'Motor Controller Status',
        key: 'motor_controller_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'HEALTHY',
          },
          {
            value: 0,
            string: 'UNHEALTHY',
          },
        ],
      },

      high_power_status: {
        name: 'High Power Status',
        key: 'high_power_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'ACTIVE',
          },
          {
            value: 0,
            string: 'OFF',
          },
        ],
      },

      // ************************************ KEYENCE ************************************ //
      keyence_1: {
        name: 'Keyence 1',
        key: 'keyence_1',
        ...keyenceCommon,
      },
      keyence_2: {
        name: 'Keyence 2',
        key: 'keyence_2',
        ...keyenceCommon,
      },

      // ************************************ POWER ************************************ //
      power_line_resistance: {
        name: 'Power Line Resistance',
        key: 'power_line_resistance',
        format: 'integer',
        type: 'resistance',
        unit: 'kΩ',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 0.1,
        sampling_time: 500,
      },
    },
  },

  pod_2024: {
    id: 'pod_2024',
    name: 'Poddington',
    operationMode: 'LEVITATION_ONLY',
    measurements: {
      // ************************************ ACCELEROMETERS ************************************ //
      accelerometer_1: {
        name: 'Accelerometer 1',
        key: 'accelerometer_1',
        ...accelerometerCommon,
      },
      accelerometer_2: {
        name: 'Accelerometer 2',
        key: 'accelerometer_2',
        ...accelerometerCommon,
      },
      accelerometer_3: {
        name: 'Accelerometer 3',
        key: 'accelerometer_3',
        ...accelerometerCommon,
      },
      accelerometer_4: {
        name: 'Accelerometer 4',
        key: 'accelerometer_4',
        ...accelerometerCommon,
      },
      accelerometer_avg: {
        name: 'Accelerometer Average',
        key: 'accelerometer_avg',
        ...accelerometerCommon,
      },

      // ************************************ NAVIGATION ************************************ //
      displacement: {
        name: 'Displacement',
        key: 'displacement',
        format: 'float',
        type: 'motion',
        unit: 'm',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 0,
        sampling_time: accelerometerCommon.sampling_time,
      },
      velocity: {
        name: 'Velocity',
        key: 'velocity',
        format: 'float',
        type: 'motion',
        unit: 'm/s',
        limits: {
          critical: {
            low: 0,
            high: 50,
          },
        },
        rms_noise: 0,
        sampling_time: accelerometerCommon.sampling_time,
      },
      acceleration: {
        name: 'Acceleration',
        key: 'acceleration',
        format: 'float',
        type: 'motion',
        unit: 'm/s²',
        limits: {
          critical: {
            low: 0,
            high: 5,
          },
        },
        rms_noise: accelerometerCommon.rms_noise,
        sampling_time: accelerometerCommon.sampling_time,
      },

      // ************************************ PRESSURE ************************************ //
      pressure_back_pull: {
        name: 'Pressure – Back Pull',
        key: 'pressure_back_pull',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_front_pull: {
        name: 'Pressure – Front Pull',
        key: 'pressure_front_pull',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_front_push: {
        name: 'Pressure – Front Push',
        key: 'pressure_front_push',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_back_push: {
        name: 'Pressure – Back Push',
        key: 'pressure_back_push',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 5.5,
          },
          warning: {
            low: -0.19,
            high: 5.2,
          },
        },
      },
      pressure_brakes_reservoir: {
        name: 'Pressure – Brakes Reservoir',
        key: 'pressure_brakes_reservoir',
        ...pressureCommon,
        limits: {
          critical: {
            low: 3,
            high: 7.4,
          },
          warning: {
            low: 3.5,
            high: 6.9,
          },
        },
      },
      pressure_active_suspension_reservoir: {
        name: 'Pressure – Active Suspension Reservoir',
        key: 'pressure_active_suspension_reservoir',
        ...pressureCommon,
        limits: {
          critical: {
            low: 3,
            high: 7.4,
          },
          warning: {
            low: 3.5,
            high: 6.9,
          },
        },
      },
      pressure_front_brake: {
        name: 'Pressure – Front Brake',
        key: 'pressure_front_brake',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 4.2,
          },
          warning: {
            low: -0.19,
            high: 4,
          },
        },
      },
      pressure_back_brake: {
        name: 'Pressure – Back Brake',
        key: 'pressure_back_brake',
        ...pressureCommon,
        limits: {
          critical: {
            low: -0.2,
            high: 4.2,
          },
          warning: {
            low: -0.19,
            high: 4,
          },
        },
      },

      // ************************************ THERMISTORS ************************************ //
      thermistor_1: {
        name: 'Thermistor 1',
        key: 'thermistor_1',
        ...thermistorCommon,
      },
      thermistor_2: {
        name: 'Thermistor 2',
        key: 'thermistor_2',
        ...thermistorCommon,
      },
      thermistor_3: {
        name: 'Thermistor 3',
        key: 'thermistor_3',
        ...thermistorCommon,
      },
      thermistor_4: {
        name: 'Thermistor 4',
        key: 'thermistor_4',
        ...thermistorCommon,
      },
      thermistor_5: {
        name: 'Thermistor 5',
        key: 'thermistor_5',
        ...thermistorCommon,
      },
      thermistor_6: {
        name: 'Thermistor 6',
        key: 'thermistor_6',
        ...thermistorCommon,
      },
      thermistor_7: {
        name: 'Thermistor 7',
        key: 'thermistor_7',
        ...thermistorCommon,
      },
      thermistor_8: {
        name: 'Thermistor 8',
        key: 'thermistor_8',
        ...thermistorCommon,
      },
      thermistor_9: {
        name: 'Thermistor 9',
        key: 'thermistor_9',
        ...thermistorCommon,
      },
      thermistor_10: {
        name: 'Thermistor 10',
        key: 'thermistor_10',
        ...thermistorCommon,
      },
      thermistor_11: {
        name: 'Thermistor 11',
        key: 'thermistor_11',
        ...thermistorCommon,
      },
      thermistor_12: {
        name: 'Thermistor 12',
        key: 'thermistor_12',
        ...thermistorCommon,
      },

      // ************************************ HALL EFFECTS ************************************ //
      hall_effect_1: {
        name: 'Hall Effect 1',
        key: 'hall_effect_1',
        ...hallEffectCommon,
      },
      hall_effect_2: {
        name: 'Hall Effect 2',
        key: 'hall_effect_2',
        ...hallEffectCommon,
      },

      // ************************************ STATUS ************************************ //
      brake_clamp_status: {
        name: 'Brake Clamp Status',
        key: 'brake_clamp_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'CLAMPED',
          },
          {
            value: 0,
            string: 'UNCLAMPED',
          },
        ],
      },
      pod_raised_status: {
        name: 'Pod Raised Status',
        key: 'pod_raised_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'RAISED',
          },
          {
            value: 0,
            string: 'LOWERED',
          },
        ],
      },

      battery_status: {
        name: 'Battery Status',
        key: 'battery_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'HEALTHY',
          },
          {
            value: 0,
            string: 'UNHEALTHY',
          },
        ],
      },

      motor_controller_status: {
        name: 'Motor Controller Status',
        key: 'motor_controller_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'HEALTHY',
          },
          {
            value: 0,
            string: 'UNHEALTHY',
          },
        ],
      },

      high_power_status: {
        name: 'High Power Status',
        key: 'high_power_status',
        format: 'enum',
        type: 'status',
        unit: 'state',
        enumerations: [
          {
            value: 1,
            string: 'ACTIVE',
          },
          {
            value: 0,
            string: 'OFF',
          },
        ],
      },

      // ************************************ KEYENCE ************************************ //
      keyence_1: {
        name: 'Keyence 1',
        key: 'keyence_1',
        ...keyenceCommon,
      },
      keyence_2: {
        name: 'Keyence 2',
        key: 'keyence_2',
        ...keyenceCommon,
      },

      // ************************************ POWER ************************************ //
      power_line_resistance: {
        name: 'Power Line Resistance',
        key: 'power_line_resistance',
        format: 'integer',
        type: 'resistance',
        unit: 'kΩ',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 0.1,
        sampling_time: 500,
      },

      // ************************************ LEVITATION ************************************ //
      levitation_height_1: {
        name: 'Levitation Height 1',
        key: 'levitation_height_1',
        ...levitationHeightCommon,
      },
      levitation_height_2: {
        name: 'Levitation Height 2',
        key: 'levitation_height_2',
        ...levitationHeightCommon,
      },
      levitation_height_3: {
        name: 'Levitation Height 3',
        key: 'levitation_height_3',
        ...levitationHeightCommon,
      },
      levitation_height_4: {
        name: 'Levitation Height 4',
        key: 'levitation_height_4',
        ...levitationHeightCommon,
      },
      levitation_height_lateral_1: {
        name: 'Levitation Height Lateral 1',
        key: 'levitation_height_lateral_1',
        format: 'float',
        type: 'levitation',
        unit: 'mm',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 2, // from Time-of-Flight datasheet
        sampling_time: 500,
      },
      levitation_height_lateral_2: {
        name: 'Levitation Height 2',
        key: 'levitation_height_lateral_2',
        format: 'float',
        type: 'levitation',
        unit: 'mm',
        limits: {
          critical: {
            low: 0,
            high: 100,
          },
        },
        rms_noise: 2, // from Time-of-Flight datasheet
        sampling_time: 500,
      },
    },
  },
};

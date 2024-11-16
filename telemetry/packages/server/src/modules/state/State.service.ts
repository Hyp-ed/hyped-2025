import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import { getStateType } from '@hyped/telemetry-constants';
import { Point } from '@influxdata/influxdb-client';
import { Inject, Injectable, type LoggerService } from '@nestjs/common';
import { MqttService } from 'nest-mqtt';
import { type StateUpdate, StateUpdateSchema } from './StateUpdate.types';
import { StateUpdateValidationError } from './errors/MeasurementReadingValidationError';

@Injectable()
export class StateService {
  constructor(
    @Logger()
    private readonly logger: LoggerService,
    private influxService: InfluxService,
    @Inject(MqttService) private readonly mqttService: MqttService,
  ) {}

  public addStateReading(props: StateUpdate) {
    const validatedState = this.validateStateUpdate(props);

    const { podId, value: state, timestamp } = validatedState;
    const stateType = getStateType(state);

    // If we want to add state to the OpenMCT dashboard, we can do it here

    // Then save it to the database
    const point = new Point('state')
      .timestamp(timestamp)
      .tag('podId', podId)
      .tag('stateType', stateType)
      .stringField('state', state);

    try {
      this.influxService.telemetryWrite.writePoint(point);

      this.logger.debug(
        `Added state ${props.podId}: ${props.value} (Type: ${stateType})`,
        StateService.name,
      );
    } catch (e) {
      this.logger.error(
        `Failed to add state ${props.podId}: ${props.value} (Type: ${stateType})`,
        e,
        StateService.name,
      );
    }
  }

  /**
   * Validates a state update.
   * @param props The state update to validate
   * @returns The validated state update, or throws an error if invalid
   */
  private validateStateUpdate(props: StateUpdate) {
    const result = StateUpdateSchema.safeParse(props);

    if (!result.success) {
      throw new StateUpdateValidationError(result.error.message);
    }

    return result.data;
  }
}

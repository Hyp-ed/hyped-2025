import type { InfluxRow } from '@/modules/common/types/InfluxRow';
import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import { ACTIVE_STATES } from '@hyped/telemetry-constants';
import type {
  LaunchTimeResponse,
  StateResponse,
} from '@hyped/telemetry-types/dist/server/responses';
import { flux } from '@influxdata/influxdb-client';
import { HttpException, Injectable, type LoggerService } from '@nestjs/common';
import { INFLUX_TELEMETRY_BUCKET } from '../core/config';

interface InfluxStateRow extends InfluxRow {
  stateType: string;
}

@Injectable()
export class PublicDataService {
  constructor(
    @Logger()
    private readonly logger: LoggerService,
    private influxService: InfluxService,
  ) {}

  /**
   * Get the current state of a pod.
   * @param podId The pod's ID.
   * @returns The current state and the previous state of the pod.
   */
  public async getState(podId: string): Promise<StateResponse> {
    // Get the last state reading from InfluxDB (measurement name should be 'state')
    const query = flux`
      from(bucket: "${INFLUX_TELEMETRY_BUCKET}")
        |> range(start: -1d)
        |> filter(fn: (r) => r["_measurement"] == "state")
        |> filter(fn: (r) => r["podId"] == "${podId}")
        |> group()
        |> sort(columns: ["_time"], desc: true)
        |> limit(n: 2)
    `;

    try {
      const data =
        await this.influxService.query.collectRows<InfluxStateRow>(query);

      return {
        currentState: data[0]
          ? {
              state: data[0]['_value'],
              timestamp: new Date(data[0]['_time']).getTime(),
              stateType: data[0]['stateType'],
            }
          : null,
        previousState: data[1]
          ? {
              state: data[1]['_value'],
              timestamp: new Date(data[1]['_time']).getTime(),
              stateType: data[1]['stateType'],
            }
          : null,
      };
    } catch (e: unknown) {
      this.logger.error(
        `Failed to get historical reading for ${podId}'s state`,
        e,
        PublicDataService.name,
      );
      throw new HttpException("Couldn't get pod's state", 500);
    }
  }

  /**
   * Get the launch time of a pod.
   * We consider the launch time to be when the pod's state changes from "READY" to "ACCELERATING", and we must currently be in active state or "STOPPED".
   * @param podId The pod's ID.
   * @returns The launch time of the pod.
   */
  public async getLaunchTime(podId: string): Promise<LaunchTimeResponse> {
    const currentState = await this.getState(podId);

    // If the pod is not in an active state or stopped, launch time isn't defined
    if (
      currentState.currentState === null ||
      !(
        Object.keys(ACTIVE_STATES).includes(currentState.currentState.state) ||
        currentState.currentState.state === 'STOPPED'
      )
    ) {
      return {
        launchTime: null,
      };
    }

    // Get the last "ACCELERATING" state reading from InfluxDB
    const query = flux`
      from(bucket: "${INFLUX_TELEMETRY_BUCKET}")
        |> range(start: -1d)
        |> filter(fn: (r) => r["_measurement"] == "state")
        |> filter(fn: (r) => r["podId"] == "${podId}")
        |> filter(fn: (r) => r["_value"] == "ACCELERATING")
        |> group()
        |> sort(columns: ["_time"], desc: true)
        |> limit(n: 1)
    `;

    try {
      const data =
        await this.influxService.query.collectRows<InfluxStateRow>(query);
      const launchTime = new Date(data[0]['_time']).getTime();

      return {
        launchTime,
      };
    } catch (e: unknown) {
      this.logger.error(
        `Failed to get launch time for ${podId}`,
        e,
        PublicDataService.name,
      );
      throw new HttpException("Couldn't get pod's launch time", 500);
    }
  }
}

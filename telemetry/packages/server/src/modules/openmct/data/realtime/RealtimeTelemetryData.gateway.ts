import { Logger } from '@/modules/logger/Logger.decorator';
import type { MeasurementReading } from '@/modules/measurement/MeasurementReading.types';
import { socket as socketConstants } from '@hyped/telemetry-constants';
import type { LoggerService } from '@nestjs/common';
import {
  ConnectedSocket,
  MessageBody,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from '@nestjs/websockets';
import type { Server, Socket } from 'socket.io';

@WebSocketGateway({
  path: '/openmct/data/realtime',
  cors: {
    origin: '*',
  },
})
export class RealtimeTelemetryDataGateway {
  @WebSocketServer()
  socket: Server;

  constructor(
    @Logger()
    private readonly logger: LoggerService,
  ) {}

  @SubscribeMessage(socketConstants.EVENTS.SUBSCRIBE_TO_MEASUREMENT)
  async subscribeToMeasurement(
    @MessageBody() measurementRoom: string,
    @ConnectedSocket() client: Socket,
  ) {
    await client.join(measurementRoom);
  }

  @SubscribeMessage(socketConstants.EVENTS.UNSUBSCRIBE_FROM_MEASUREMENT)
  async unsubscribeFromMeasurement(
    @MessageBody() measurementRoom: string,
    @ConnectedSocket() client: Socket,
  ) {
    await client.leave(measurementRoom);
  }

  sendMeasurementReading(props: MeasurementReading) {
    const { podId, measurementKey, value, timestamp } = props;

    const measurementRoom = socketConstants.getMeasurementRoomName(
      podId,
      measurementKey,
    );
    this.socket.to(measurementRoom).emit(socketConstants.MEASUREMENT_EVENT, {
      podId,
      measurementKey,
      value,
      // convert timestamp from nanoseconds to milliseconds
      timestamp: Math.floor(Number(timestamp) / 1e6),
    });

    this.logger.debug(
      `Sending realtime ${value} to ${measurementRoom}`,
      RealtimeTelemetryDataGateway.name,
    );
  }
}

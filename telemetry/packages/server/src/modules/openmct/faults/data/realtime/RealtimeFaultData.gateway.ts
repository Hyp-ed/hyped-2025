import { Logger } from '@/modules/logger/Logger.decorator';
import { socket as socketConstants } from '@hyped/telemetry-constants';
import { OpenMctFault } from '@hyped/telemetry-types';
import { LoggerService } from '@nestjs/common';
import {
  ConnectedSocket,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from '@nestjs/websockets';
import { Server, Socket } from 'socket.io';

const FAULT_ROOM = 'faults';

@WebSocketGateway({
  path: '/openmct/faults/realtime',
  cors: {
    origin: '*',
  },
})
export class RealtimeFaultDataGateway {
  @WebSocketServer()
  socket: Server;

  constructor(
    @Logger()
    private readonly logger: LoggerService,
  ) {}

  @SubscribeMessage(socketConstants.EVENTS.SUBSCRIBE_TO_FAULTS)
  async subscribeToFaults(@ConnectedSocket() client: Socket) {
    await client.join(FAULT_ROOM);
  }

  @SubscribeMessage(socketConstants.EVENTS.UNSUBSCRIBE_FROM_FAULTS)
  async unsubscribeFromFaults(@ConnectedSocket() client: Socket) {
    await client.leave(FAULT_ROOM);
  }

  sendFault(fault: OpenMctFault) {
    this.socket.to(FAULT_ROOM).emit(socketConstants.FAULT_EVENT, {
      fault,
    });

    this.logger.debug(
      `Sending fault with id ${fault.fault.id}`,
      RealtimeFaultDataGateway.name,
    );
  }
}

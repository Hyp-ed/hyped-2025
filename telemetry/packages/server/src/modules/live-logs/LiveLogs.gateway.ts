import {
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from '@nestjs/websockets';
import { Server, Socket } from 'socket.io';

@WebSocketGateway({
  path: '/live-logs',
  cors: {
    origin: '*',
  },
})
export class LiveLogsGateway {
  @WebSocketServer()
  socket: Server;

  @SubscribeMessage('send-log')
  handleMessage(_client: Socket, payload: unknown) {
    this.socket.emit('log', payload);
  }
}

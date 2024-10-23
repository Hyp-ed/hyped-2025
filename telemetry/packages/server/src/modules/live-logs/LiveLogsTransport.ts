import Transport from 'winston-transport';
import { io } from 'socket.io-client';
import { Socket } from 'socket.io';
import { DefaultEventsMap } from 'socket.io/dist/typed-events';

export class LiveLogsTransport extends Transport {
  constructor() {
    super();
    this.init();
  }

  private socket: Socket<DefaultEventsMap, DefaultEventsMap>;

  // iniitalise the socket connection to the server
  init() {
    const socket = io('http://localhost:3000', {
      path: '/live-logs',
    });

    this.socket = socket as unknown as Socket<
      DefaultEventsMap,
      DefaultEventsMap
    >;
  }

  log(info: unknown, callback: () => void) {
    setImmediate(() => {
      this.socket.emit('send-log', info);
    });

    callback();
  }
}

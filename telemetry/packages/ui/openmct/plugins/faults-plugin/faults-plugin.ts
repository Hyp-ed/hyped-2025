/* eslint-disable @typescript-eslint/unbound-method */

import { OpenMCT } from 'openmct/dist/openmct';
import { HistoricalFaultsProvider } from './historical-faults-provider';
import { RealtimeFaultsProvider } from './realtime-faults-provider';
import { http } from 'openmct/core/http';
import { OpenMctFault } from '@hyped/telemetry-types';
import { log } from '@/lib/logger';

/**
 * The Faults plugin for Open MCT.
 * Provides fault management for telemetry data by querying the telemetry server for faults.
 * @returns The faults plugin function.
 */
export function FaultsPlugin() {
  return function install(openmct: OpenMCT) {
    // Add the (built-in) Fault Management plugin to Open MCT
    openmct.install(openmct.plugins.FaultManagement());

    const realtimeProvider = RealtimeFaultsProvider();
    const historicalProvider = HistoricalFaultsProvider();

    // Add our fault providers to Open MCT
    openmct.faults.addProvider({
      supportsRequest: historicalProvider.supportsRequest,
      supportsSubscribe: realtimeProvider.supportsSubscribe,
      subscribe: realtimeProvider.subscribe,
      request: historicalProvider.request,
      acknowledgeFault,
      shelveFault,
    });
  };
}

/**
 * Sends an acknowledgement for a fault to the server.
 * @param fault The Open MCT fault object.
 * @param comment The comment to send with the acknowledgement.
 * In the future we could also do something with the comments, but for now we will just log them.
 */
async function acknowledgeFault(
  fault: OpenMctFault['fault'],
  { comment }: { comment: string },
) {
  const url = `openmct/faults/acknowledge`;
  await http.post(url, {
    body: JSON.stringify({
      faultId: fault.id,
      comment,
    }),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  log(
    `Acknowledged fault with id ${fault.id}.${comment ? ` Comment: ${comment}` : ''}`,
  );
}

/**
 * Sends a shelve request for a fault to the server.
 * @param fault The Open MCT fault object.
 * @param shelveDuration The duration to shelve the fault for (in milliseconds).
 * @param comment The comment to send with the acknowledgement.
 * In the future we could also do something with the comments, but for now we will just log them.
 */
async function shelveFault(
  fault: OpenMctFault['fault'],
  {
    shelveDuration,
    comment,
    shelved = false,
  }: {
    shelveDuration: number;
    comment: string;
    shelved: boolean;
  },
) {
  const url = `openmct/faults/shelve`;
  await http.post(url, {
    body: JSON.stringify({
      faultId: fault.id,
      shelved,
      shelveDuration,
      comment,
    }),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  log(
    `${shelved ? 'Shelving' : 'Unshelving'} fault with id ${fault.id} for ${shelveDuration} seconds.${comment ? ` Comment: ${comment}` : ''}`,
  );
}

import { http } from 'openmct/core/http';

/**
 * Logs a message to the UI log
 * @param message The message to log
 * @param podId The pod ID to log with (optional)
 */
export const log = (message: string, podId?: string) => {
  if (podId) {
    // eslint-disable-next-line no-console
    console.log(`[LOG] (${podId}) ${message}`);
    void http.post(`logs/${podId}`, {
      body: JSON.stringify({ message }),
      headers: {
        'content-type': 'application/json',
      },
    });
  } else {
    // eslint-disable-next-line no-console
    console.log(`[LOG] ${message}`);
    void http.post(`logs`, {
      body: JSON.stringify({ message }),
      headers: {
        'content-type': 'application/json',
      },
    });
  }
};

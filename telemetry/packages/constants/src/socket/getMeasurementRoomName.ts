export function getMeasurementRoomName(
  podId: string,
  measurementKey: string,
): string {
  return `${podId}/measurement/${measurementKey}`;
}

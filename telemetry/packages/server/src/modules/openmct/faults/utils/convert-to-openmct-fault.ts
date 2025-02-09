import type { OpenMctFault } from "@hyped/telemetry-types";
import { nanoid } from "nanoid";
import type { Fault } from "../Fault.service";

/**
 * Converts a fault to an Open MCT fault object.
 * @param fault The fault to convert
 * @returns The Open MCT fault object
 */
export function convertToOpenMctFault(fault: Fault): OpenMctFault {
	const { measurement, tripReading, level } = fault;

	const namespace = `/${tripReading.podId}/${measurement.id}`;

	return {
		type: "global-alarm-status",
		fault: {
			id: `${namespace}-${nanoid()}`,
			name: `${measurement.label} is out of range`,
			namespace,
			seqNum: 0,
			severity: level,
			shortDescription: "",
			shelved: false,
			acknowledged: false,
			triggerTime: tripReading.timestamp.toString(),
			triggerValueInfo: {
				value: tripReading.value,
				rangeCondition: level,
				monitoringResult: level,
			},
			currentValueInfo: {
				value: tripReading.value,
				rangeCondition: level,
				monitoringResult: level,
			},
		},
	};
}

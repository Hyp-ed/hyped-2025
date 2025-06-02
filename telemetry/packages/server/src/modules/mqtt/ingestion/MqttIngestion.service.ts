import type { StateService } from '@/modules/state/State.service';
import type { MeasurementService } from '@/modules/telemetry/Measurement.service';
import { type PodStateType, podIds } from '@hyped/telemetry-constants';
import { currentTime } from '@influxdata/influxdb-client';
import { Injectable } from '@nestjs/common';
import { Params, Payload, Subscribe } from 'nest-mqtt';
import { MqttIngestionError } from './errors/MqttIngestionError';

@Injectable()
export class MqttIngestionService {
	constructor(
		private measurementService: MeasurementService,
		private stateService: StateService,
	) {}

	@Subscribe('hyped/+/measurement/+')
	async getMeasurementReading(
		@Params() rawParams: string[],
		@Payload() rawValue: number, // TODOLater: check that this is correct
	) {
		const timestamp = currentTime.nanos();
		const podId = rawParams[0];
		const measurementKey = rawParams[1];
		const value = rawValue;

		this.validateMqttMessage({ podId, measurementKey, value });
		this.validatePodId(podId);

		await this.measurementService.addMeasurementReading({
			podId,
			measurementKey,
			value,
			timestamp,
		});
	}

	@Subscribe('hyped/+/state')
	getStateReading(
		@Params() rawParams: string[],
		@Payload() rawValue: PodStateType,
	) {
		const timestamp = currentTime.nanos();
		const podId = rawParams[0];
		const value = rawValue;

		this.validateMqttMessage({ podId, measurementKey: 'state', value });
		this.validatePodId(podId);

		this.stateService.addStateReading({
			podId,
			value,
			timestamp,
		});
	}

	private validateMqttMessage({
		podId,
		measurementKey,
		value,
	}: {
		podId: string;
		measurementKey: string;
		value: unknown;
	}) {
		if (!podId || !measurementKey || value === undefined) {
			throw new MqttIngestionError('Invalid MQTT message');
		}
	}

	private validatePodId(podId: string) {
		if (!podIds.includes(podId)) {
			throw new MqttIngestionError('Invalid pod ID');
		}
	}
}

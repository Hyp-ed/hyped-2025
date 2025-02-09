import type { InfluxService } from '@/modules/influx/Influx.service';
import { Logger } from '@/modules/logger/Logger.decorator';
import type { MeasurementReading } from '@/modules/telemetry/MeasurementReading.types';
import type { FaultLevel } from '@hyped/telemetry-constants';
import type { Measurement, OpenMctFault, Unpacked } from '@hyped/telemetry-types';
import type { HistoricalFaults } from '@hyped/telemetry-types/dist/openmct/openmct-fault.types';
import { Point } from '@influxdata/influxdb-client';
import { Injectable, type LoggerService } from '@nestjs/common';
import type { HistoricalFaultDataService } from './data/historical/HistoricalFaultData.service';
import type { RealtimeFaultDataGateway } from './data/realtime/RealtimeFaultData.gateway';
import { convertToOpenMctFault } from './utils/convertToOpenMctFault';

export type Fault = {
	level: FaultLevel;
	measurement: Measurement;
	tripReading: MeasurementReading;
};

@Injectable()
export class FaultService {
	constructor(
		@Logger()
		private readonly logger: LoggerService,
		private influxService: InfluxService,
		private historicalService: HistoricalFaultDataService,
		private realtimeService: RealtimeFaultDataGateway,
	) {}

	/**
	 * Adds a limit breach fault to the database and sends it to the client.
	 * @param fault The fault to add
	 */
	public async addLimitBreachFault(fault: Fault) {
		const { measurement, tripReading } = fault;

		const possibleExistingFaults =
			await this.historicalService.getHistoricalFaults({
				podId: tripReading.podId,
				measurementKey: measurement.id,
			});

		// If there's an existing fault, update it instead of creating a new one
		if (possibleExistingFaults && possibleExistingFaults?.length > 0) {
			const existingFault = possibleExistingFaults[0];
			this.logger.debug(
				`Found existing fault ${existingFault.openMctFault.fault.id}, updating`,
				FaultService.name,
			);
			this.updateExistingFault(existingFault, tripReading);
			return;
		}

		const openMctFault = convertToOpenMctFault(fault);
		this.saveFault(fault, openMctFault);
		this.realtimeService.sendFault(openMctFault);
	}

	/**
	 * Acknowledges a fault in the database and sends it to the client.
	 * @param faultId The id of the fault to acknowledge
	 * @param comment The comment sent with the acknowledgement
	 */
	public async acknowledgeFault(faultId: string, comment?: string) {
		this.logger.debug(
			`Acknowledging fault with id ${faultId}. ${comment ? `Comment: ${comment}` : ''}`,
			FaultService.name,
		);

		// Get the fault from the database
		const possibleFault = await this.historicalService.getHistoricalFaults({
			faultId,
		});

		if (!possibleFault || possibleFault.length === 0) {
			this.logger.error(
				`Fault with id ${faultId} not found`,
				FaultService.name,
			);
			return;
		}

		const fault = possibleFault[0];
		const updatedFault = fault.openMctFault;
		updatedFault.fault.acknowledged = true;

		const now = new Date(Date.now());

		const point = new Point('fault')
			.timestamp(now)
			.tag('faultId', updatedFault.fault.id)
			.tag('podId', fault.podId)
			.tag('measurementKey', fault.measurementKey)
			.stringField('fault', JSON.stringify(updatedFault));

		try {
			this.influxService.faultsWrite.writePoint(point);

			this.logger.debug(
				`Acknowledged fault with id ${updatedFault.fault.id}`,
				FaultService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to acknowledge fault with id ${updatedFault.fault.id}`,
				e,
				FaultService.name,
			);
		}

		this.realtimeService.sendFault(updatedFault);
	}

	/**
	 * Shelves or unshelves a fault in the database and sends it to the client.
	 * @param faultId The id of the fault to shelve
	 * @param shelved Whether to shelve or unshelve the fault
	 * @param shelveDuration The duration to shelve the fault for (in milliseconds)
	 * @param comment The comment sent with the shelve
	 */
	public async shelveFault(
		faultId: string,
		shelved: boolean,
		shelveDuration: number,
		comment?: string,
	) {
		this.logger.debug(
			`Shelving fault with id ${faultId} for ${shelveDuration} seconds. ${comment ? `Comment: ${comment}` : ''}`,
			FaultService.name,
		);

		// Get the fault from the database
		const possibleFault = await this.historicalService.getHistoricalFaults({
			faultId,
		});

		if (!possibleFault || possibleFault.length === 0) {
			this.logger.error(
				`Fault with id ${faultId} not found`,
				FaultService.name,
			);
			return;
		}

		const fault = possibleFault[0];
		const updatedFault = fault.openMctFault;
		updatedFault.fault.shelved = shelved;

		const now = new Date(Date.now());

		const point = new Point('fault')
			.timestamp(now)
			.tag('faultId', updatedFault.fault.id)
			.tag('podId', fault.podId)
			.tag('measurementKey', fault.measurementKey)
			.stringField('fault', JSON.stringify(updatedFault));

		// If the fault is being shelved, set a timer to unshelve it
		if (shelved) {
			this.logger.debug(
				`Setting timer to unshelve fault with id ${faultId}`,
				FaultService.name,
			);
			// This isn't ideal, since this doesn't persist if the server restarts.
			// Also, if the fault is manually unshelved before the timer runs out, the timer should be cancelled and it isn't.
			// (So the fault will be unshelved twice, once manually and once automatically. Plus, if the fault is unshelved and then shelved again, the timer won't be reset.)
			setTimeout(() => {
				this.logger.debug(
					`Automatically unshelving fault with id ${faultId} because the shelve duration has passed`,
					FaultService.name,
				);
				void this.shelveFault(faultId, false, 0, '');
			}, shelveDuration);
		}

		try {
			this.influxService.faultsWrite.writePoint(point);

			this.logger.debug(
				`Shelved fault with id ${updatedFault.fault.id}`,
				FaultService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to shelve fault with id ${updatedFault.fault.id}`,
				e,
				FaultService.name,
			);
		}

		this.realtimeService.sendFault(updatedFault);
	}

	/**
	 * Saves a fault to the database.
	 * @param fault The fault to save
	 * @param openMctFault The Open MCT fault object to save
	 */
	private saveFault(fault: Fault, openMctFault: OpenMctFault) {
		const { measurement, tripReading } = fault;

		const point = new Point('fault')
			.timestamp(tripReading.timestamp)
			.tag('faultId', openMctFault.fault.id)
			.tag('podId', tripReading.podId)
			.tag('measurementKey', measurement.id)
			// is influx the right choice? probably not - but we're already using it for telemetry
			.stringField('fault', JSON.stringify(openMctFault));

		try {
			this.influxService.faultsWrite.writePoint(point);

			this.logger.debug(
				`Adding fault with id ${openMctFault.fault.id}`,
				FaultService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to add fault {${openMctFault.fault.id}}`,
				e,
				FaultService.name,
			);
		}
	}

	/**
	 * Updates an existing fault in the database.
	 * @param influxFault The fault to update
	 * @param updatedReading The updated reading
	 */
	private updateExistingFault(
		influxFault: Unpacked<HistoricalFaults>,
		updatedReading: MeasurementReading,
	) {
		const updatedFault = influxFault.openMctFault;
		updatedFault.fault.currentValueInfo.value = updatedReading.value;

		const point = new Point('fault')
			.timestamp(updatedReading.timestamp)
			.tag('faultId', updatedFault.fault.id)
			.tag('podId', updatedReading.podId)
			.tag('measurementKey', updatedReading.measurementKey)
			.stringField('fault', JSON.stringify(updatedFault));

		try {
			this.influxService.faultsWrite.writePoint(point);

			this.logger.debug(
				`Updating fault with id ${updatedFault.fault.id}`,
				FaultService.name,
			);
		} catch (e: unknown) {
			this.logger.error(
				`Failed to update fault with id ${updatedFault.fault.id}`,
				e,
				FaultService.name,
			);
		}

		this.realtimeService.sendFault(updatedFault);
	}
}

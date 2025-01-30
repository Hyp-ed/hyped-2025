import type { HistoricalTelemetryDataService } from '@/modules/openmct/data/historical/HistoricalTelemetryData.service';
import { POD_IDS, type PodId } from '@hyped/telemetry-constants';
import type {
	LevitationHeightResponse,
	RawLevitationHeight,
} from '@hyped/telemetry-types';
import type {
	HistoricalValueResponse,
	LaunchTimeResponse,
	LevitationHeight,
	StateResponse,
} from '@hyped/telemetry-types/dist/server/responses';
import { Controller, Get, HttpException, Param, Query } from '@nestjs/common';
import type { PublicDataService } from './PublicData.service';

@Controller('pods/:podId/public-data')
export class PublicDataController {
	constructor(
		private publicDataService: PublicDataService,
		private historicalTelemetryDataService: HistoricalTelemetryDataService,
	) {}

	@Get('launch-time')
	async getLaunchTime(
		@Param('podId') podId: string,
	): Promise<LaunchTimeResponse> {
		this.validatePodId(podId);
		return this.publicDataService.getLaunchTime(podId);
	}

	@Get('state')
	async getState(@Param('podId') podId: string): Promise<StateResponse> {
		this.validatePodId(podId);
		return this.publicDataService.getState(podId);
	}

	@Get('velocity')
	async getData(
		@Param('podId') podId: string,
		@Query('start') startTimestamp: string,
		@Query('end') endTimestamp?: string,
	): Promise<HistoricalValueResponse> {
		if (!startTimestamp) {
			throw new HttpException("Missing 'start' query parameter", 400);
		}
		this.validatePodId(podId);
		return this.historicalTelemetryDataService.getHistoricalReading(
			podId,
			'velocity',
			startTimestamp,
			endTimestamp ?? new Date().getTime().toString(),
		);
	}

	@Get('displacement')
	async getDisplacement(
		@Param('podId') podId: string,
		@Query('start') startTimestamp: string,
		@Query('end') endTimestamp?: string,
	): Promise<HistoricalValueResponse> {
		if (!startTimestamp) {
			throw new HttpException("Missing 'start' query parameter", 400);
		}
		this.validatePodId(podId);
		return this.historicalTelemetryDataService.getHistoricalReading(
			podId,
			'displacement',
			startTimestamp,
			endTimestamp ?? new Date().getTime().toString(),
		);
	}

	@Get('acceleration')
	async getAcceleration(
		@Param('podId') podId: string,
		@Query('start') startTimestamp: string,
		@Query('end') endTimestamp?: string,
	): Promise<HistoricalValueResponse> {
		if (!startTimestamp) {
			throw new HttpException("Missing 'start' query parameter", 400);
		}
		this.validatePodId(podId);
		return this.historicalTelemetryDataService.getHistoricalReading(
			podId,
			'acceleration',
			startTimestamp,
			endTimestamp ?? new Date().getTime().toString(),
		);
	}

	@Get('levitation-height')
	async getLevitationHeight(
		@Param('podId') podId: string,
		@Query('start') startTimestamp: string,
		@Query('end') endTimestamp?: string,
	): Promise<LevitationHeightResponse> {
		this.validatePodId(podId);

		// TODOLater: this is basically quite bad, but we'll fix it later
		const [
			levitation_height_1,
			levitation_height_2,
			levitation_height_3,
			levitation_height_4,
			levitation_height_lateral_1,
			levitation_height_lateral_2,
		] = await Promise.all([
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_1',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_2',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_3',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_4',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_lateral_1',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
			this.historicalTelemetryDataService.getHistoricalReading(
				podId,
				'levitation_height_lateral_2',
				startTimestamp,
				endTimestamp ?? new Date().getTime().toString(),
			),
		]);

		return {
			levitation_height_1: this.convertValueToInt(levitation_height_1[0]),
			levitation_height_2: this.convertValueToInt(levitation_height_2[0]),
			levitation_height_3: this.convertValueToInt(levitation_height_3[0]),
			levitation_height_4: this.convertValueToInt(levitation_height_4[0]),
			levitation_height_lateral_1: this.convertValueToInt(
				levitation_height_lateral_1[0],
			),
			levitation_height_lateral_2: this.convertValueToInt(
				levitation_height_lateral_2[0],
			),
		} satisfies LevitationHeightResponse;
	}

	private convertValueToInt(
		levitationHeights: RawLevitationHeight,
	): LevitationHeight {
		return {
			id: levitationHeights.id,
			timestamp: levitationHeights.timestamp,
			value: Number.parseInt(levitationHeights.value),
		};
	}

	private validatePodId(podId: string) {
		if (!POD_IDS.includes(podId as PodId)) {
			throw new HttpException('Invalid pod ID', 400);
		}
	}
}

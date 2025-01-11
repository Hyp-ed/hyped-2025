import { openMctObjectTypes } from '@hyped/telemetry-constants';
import type { OpenMctObjectTypes } from '@hyped/telemetry-types';
import { Injectable } from '@nestjs/common';

@Injectable()
export class ObjectTypesService {
	/**
	 * Get the object types for Open MCT.
	 * @returns The object types for Open MCT from the constants.
	 */
	getObjectTypes(): OpenMctObjectTypes {
		return openMctObjectTypes;
	}
}

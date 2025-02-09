import type { OpenMctObjectTypes } from '@hyped/telemetry-types';
import type { telemetryTypes } from '../../pods/types';

export type OpenMctObjectTypeId = (typeof telemetryTypes)[number];

type StrictOpenMctObjectTypes = OpenMctObjectTypes &
	{ id: OpenMctObjectTypeId }[];

export const openMctObjectTypes: StrictOpenMctObjectTypes = [
	{
		id: 'temperature',
		name: 'Temperature',
		icon: 'icon-telemetry',
	},
	{
		id: 'acceleration',
		name: 'Acceleration',
		icon: 'icon-telemetry',
	},
	{
		id: 'pressure',
		name: 'Pressure',
		icon: 'icon-telemetry',
	},
	{
		id: 'hall_effect',
		name: 'Hall Effect',
		icon: 'icon-telemetry',
	},
	{
		id: 'displacement',
		name: 'Displacement',
		icon: 'icon-telemetry',
	},
	{
		id: 'velocity',
		name: 'Velocity',
		icon: 'icon-telemetry',
	},
	{
		id: 'status',
		name: 'status',
		icon: 'icon-telemetry',
	},
	{
		id: 'magnetism',
		name: 'Magnetism',
		icon: 'icon-telemetry',
	},
	{
		id: 'keyence',
		name: 'Keyence',
		icon: 'icon-telemetry',
	},
	{
		id: 'resistance',
		name: 'Resistance',
		icon: 'icon-telemetry',
	},
	{
		id: 'levitation',
		name: 'Levitation',
		icon: 'icon-telemetry',
	},
	{
		id: 'binary-status',
		name: 'Binary Status',
		icon: 'icon-telemetry',
	},
];

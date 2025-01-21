import type { OpenMCT } from 'openmct/dist/openmct';

const THIRTY_SECONDS = 30 * 1000;
const ONE_MINUTE = THIRTY_SECONDS * 2;
const FIVE_MINUTES = ONE_MINUTE * 5;
const FIFTEEN_MINUTES = FIVE_MINUTES * 3;
const THIRTY_MINUTES = FIFTEEN_MINUTES * 2;
const ONE_HOUR = THIRTY_MINUTES * 2;
const TWO_HOURS = ONE_HOUR * 2;
const ONE_DAY = ONE_HOUR * 24;

/**
 * The Conductor plugin for Open MCT.
 * @see https://github.com/nasa/openmct/blob/master/API.md#the-time-conductor
 * @returns The conductor plugin function.
 */
export function ConductorPlugin() {
	return function install(openmct: OpenMCT) {
		openmct.install(
			openmct.plugins.Conductor({
				menuOptions: [
					{
						name: 'Realtime',
						timeSystem: 'utc',
						clock: 'local',
						clockOffsets: {
							start: -FIVE_MINUTES,
							end: ONE_MINUTE,
						},
						presets: [
							{
								label: '1 Hour',
								bounds: {
									start: -ONE_HOUR,
									end: THIRTY_SECONDS,
								},
							},
							{
								label: '30 Minutes',
								bounds: {
									start: -THIRTY_MINUTES,
									end: THIRTY_SECONDS,
								},
							},
							{
								label: '15 Minutes',
								bounds: {
									start: -FIFTEEN_MINUTES,
									end: THIRTY_SECONDS,
								},
							},
							{
								label: '5 Minutes',
								bounds: {
									start: -FIVE_MINUTES,
									end: THIRTY_SECONDS,
								},
							},
							{
								label: '1 Minute',
								bounds: {
									start: -ONE_MINUTE,
									end: THIRTY_SECONDS,
								},
							},
						],
					},
					{
						name: 'Fixed',
						timeSystem: 'utc',
						bounds: {
							start: Date.now() - THIRTY_MINUTES,
							end: Date.now(),
						},
						// commonly used bounds can be stored in history
						// bounds (start and end) can accept either a milliseconds number
						// or a callback function returning a milliseconds number
						// a function is useful for invoking Date.now() at exact moment of preset selection
						presets: [
							{
								label: 'Last Day',
								bounds: {
									start: () => Date.now() - ONE_DAY,
									end: () => Date.now(),
								},
							},
							{
								label: 'Last 2 hours',
								bounds: {
									start: () => Date.now() - TWO_HOURS,
									end: () => Date.now(),
								},
							},
							{
								label: 'Last hour',
								bounds: {
									start: () => Date.now() - ONE_HOUR,
									end: () => Date.now(),
								},
							},
						],
						// maximum recent bounds to retain in conductor history
						records: 10,
						// maximum duration between start and end bounds
						// for utc-based time systems this is in milliseconds
						// limit: ONE_DAY
					},
				],
			}),
		);
	};
}

import openmct from 'openmct/dist/openmct';

import { SERVER_ENDPOINT } from './core/config';
import { ConductorPlugin } from './plugins/conductor';
import { DictionaryPlugin } from './plugins/dictionary-plugin';
import { FaultsPlugin } from './plugins/faults-plugin/faults-plugin';
import { HistoricalTelemetryPlugin } from './plugins/historical-telemetry-plugin';
import { LimitPlugin } from './plugins/limit-plugin';
import { RealtimeTelemetryPlugin } from './plugins/realtime-telemetry-plugin';

openmct.setAssetPath('/openmct-lib');

// Local storage of dashboards
openmct.install(openmct.plugins.LocalStorage());
// https://github.com/nasa/openmct/tree/master/src/plugins/staticRootPlugin
openmct.install(
	openmct.plugins.StaticRootPlugin({
		namespace: 'dashboards',
		exportUrl: 'data/dashboards.json',
	}),
);

// Time
openmct.install(openmct.plugins.UTCTimeSystem());
openmct.install(ConductorPlugin());

// Theme
openmct.install(openmct.plugins['Espresso']());

// Views
openmct.install(openmct.plugins.MyItems());
openmct.install(openmct.plugins.PlanLayout());
openmct.install(openmct.plugins.Timeline());
openmct.install(openmct.plugins.Hyperlink());
openmct.install(
	openmct.plugins.AutoflowView({
		type: 'telemetry.panel',
	}),
);
openmct.install(
	openmct.plugins.DisplayLayout({
		showAsView: ['summary-widget', 'example.imagery'],
	}),
);
openmct.install(openmct.plugins.LADTable());
openmct.install(openmct.plugins.SummaryWidget());
openmct.install(openmct.plugins.Timer());
openmct.install(openmct.plugins.Timelist());
openmct.install(openmct.plugins.BarChart());
openmct.install(openmct.plugins.ScatterPlot());

// Data
openmct.install(DictionaryPlugin());
openmct.install(HistoricalTelemetryPlugin());
openmct.install(RealtimeTelemetryPlugin());

// Limits/faults
openmct.install(LimitPlugin());
openmct.install(FaultsPlugin());

// Server connection indicator
openmct.install(
	openmct.plugins.URLIndicator({
		url: SERVER_ENDPOINT + '/ping',
		iconClass: 'icon-check',
		interval: 1000,
		label: 'Telemetry Server',
	}),
);

// TODOLater: extract to utils
function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

// Wait for all plugins to be installed before starting (this is actually needed)
void sleep(1000).then(() => {
	openmct.start();
});

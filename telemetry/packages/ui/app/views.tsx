import { Bug, GitBranch, LineChart, Terminal } from 'lucide-react';
import { LogViewer } from './components/views/log-viewer';
import { OpenMCT } from './components/views/openmct';
import { DebugView } from './components/views/debug-view';
import { StateMachine } from './components/views/state-machine';

/**
 * The views that can be rendered in the LHS of the GUI.
 */
export const VIEWS = {
  OPEN_MCT: {
    name: 'OpenMCT',
    component: <OpenMCT />,
    icon: <LineChart width={18} />,
  },
  LOGS: {
    name: 'Logs',
    component: <LogViewer />,
    icon: <Terminal width={18} />,
  },
  DEBUG_VIEW: {
    name: 'Debug View',
    component: <DebugView />,
    icon: <Bug width={18} />,
  },
  STATE: {
    name: 'State',
    component: <StateMachine />,
    icon: <GitBranch width={18} />,
  },
} as const;

export type View = (typeof VIEWS)[keyof typeof VIEWS];
export type ViewOption = keyof typeof VIEWS;

// Make an object with keys of the view keys and value of the view keys
export const VIEW_OPTIONS = (Object.keys(VIEWS) as ViewOption[]).reduce(
  (acc, key) => {
    acc[key] = key;
    return acc;
  },
  {} as Record<ViewOption, ViewOption>,
);
export const VIEW_KEYS = Object.keys(VIEWS) as ViewOption[];

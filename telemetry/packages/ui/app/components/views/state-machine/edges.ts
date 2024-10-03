import { EdgeMarkerType, MarkerType } from 'reactflow';
import { ModeType, PodStateType } from '@hyped/telemetry-constants';
import { CustomEdgeType } from './types';
import { isEnabledState } from './utils';
import { MODES } from '@hyped/telemetry-constants';

// defines the arrow marker for the edges
export const arrow: EdgeMarkerType = {
  type: MarkerType.Arrow,
  width: 32,
  height: 32,
  strokeWidth: 0.5,
};

export const edges: CustomEdgeType[] = [
  {
    id: 'idle-calibrate',
    source: 'idle',
    target: 'calibrate',
    sourceHandle: 'top',
    targetHandle: 'bottom',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'calibrate-precharge',
    source: 'calibrate',
    target: 'precharge',
    sourceHandle: 'top',
    targetHandle: 'bottom',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'precharge-ready-for-levitation',
    source: 'precharge',
    target: 'ready-for-levitation',
    sourceHandle: 'top',
    targetHandle: 'bottom',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'ready-for-levitation-begin-levitation',
    source: 'ready-for-levitation',
    target: 'begin-levitation',
    sourceHandle: 'top-right',
    targetHandle: 'bottom-left',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'begin-levitation-ready-for-launch',
    source: 'begin-levitation',
    target: 'ready-for-launch',
    sourceHandle: 'top-right',
    targetHandle: 'bottom-left',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'ready-for-launch-accelerate',
    source: 'ready-for-launch',
    target: 'accelerate',
    sourceHandle: 'top-top',
    targetHandle: 'top-top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'accelerate-lim-brake',
    source: 'accelerate',
    target: 'lim-brake',
    sourceHandle: 'bottom-bottom',
    targetHandle: 'top-top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'lim-brake-friction-brake',
    source: 'lim-brake',
    target: 'friction-brake',
    sourceHandle: 'bottom-bottom',
    targetHandle: 'top-top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'friction-brake-stop-levitation',
    source: 'friction-brake',
    target: 'stop-levitation',
    sourceHandle: 'right',
    targetHandle: 'left',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  //
  {
    id: 'stop-levitation-stopped',
    source: 'stop-levitation',
    target: 'stopped',
    sourceHandle: 'bottom',
    targetHandle: 'top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'stopped-battery-recharge',
    source: 'stopped',
    target: 'battery-recharge',
    sourceHandle: 'bottom',
    targetHandle: 'top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'battery-recharge-capacitor-discharge',
    source: 'battery-recharge',
    target: 'capacitor-discharge',
    sourceHandle: 'bottom',
    targetHandle: 'top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'capacitor-discharge-safe',
    source: 'capacitor-discharge',
    target: 'safe',
    sourceHandle: 'bottom',
    targetHandle: 'top',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
  {
    id: 'failure-braking-capacitor-discharge',
    source: 'failure-braking',
    target: 'capacitor-discharge',
    sourceHandle: 'right',
    targetHandle: 'left',
    type: 'smoothstep',
    pathOptions: {
      borderRadius: 20,
    },
    markerEnd: arrow,
  },
];

// Generates edge definitions for a given mode of operation
export const writeEdges = (mode: ModeType): CustomEdgeType[] => {
  return edges
    .filter((edge) =>
      isEnabledState(
        mode,
        edge.source.replace(/-/g, '_').toUpperCase() as PodStateType,
      ),
    )
    .map((edge, i, arr) => {
      if (i < arr.length - 2) {
        const regex = new RegExp(`${arr[i + 1].source}$`);
        if (!regex.test(edge.id)) {
          edge.id = `${edge.source}-${arr[i + 1].source}`;
          edge.target = arr[i + 1].source;
        }
      }
      return {
        ...edge,
        ...(edge.sourceHandle.includes('-') && mode != MODES.ALL_SYSTEMS_ON
          ? {
              sourceHandle: edge.sourceHandle.split('-')[1],
              targetHandle: edge.targetHandle.split('-')[1],
            }
          : {
              sourceHandle: edge.sourceHandle.split('-')[0],
              targetHandle: edge.targetHandle.split('-')[0],
            }),
      };
    });
};

import { Node, Position, Edge, SmoothStepPathOptions } from 'reactflow';

/**
 * Data passed to custom nodes
 */
export type NodeDataType = {
  label: string;
  sourcePositions?: {
    position: Position;
    id: string;
  }[];
  targetPositions?: {
    position: Position;
    id: string;
  }[];
  active?: boolean;
};

export type CustomNodeType = Omit<Node, 'data'> & {
  data: NodeDataType & {
    sourcePositions: {
      position: Position;
      id: string;
    }[];
  };
};

export type CustomEdgeType = Omit<Edge, 'sourceHandle' | 'targetHandle'> & {
  sourceHandle: string;
  targetHandle: string;
} & {
  pathOptions?: SmoothStepPathOptions;
};

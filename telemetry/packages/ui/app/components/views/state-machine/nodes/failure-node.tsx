import { memo } from 'react';
import { Handle, NodeProps } from 'reactflow';
import { BASE_NODE_STYLES } from './styles';
import { cn } from '@/lib/utils';
import { NodeDataType } from '../types';

const FailureNode = memo(
  ({
    data,
  }: Omit<NodeProps, 'data'> & {
    data: NodeDataType;
  }) => (
    <>
      {data.targetPositions &&
        data.targetPositions.map(({ position, id }) => (
          <Handle key={id} type="target" position={position} id={id} />
        ))}
      <div
        className={cn(
          BASE_NODE_STYLES,
          data.active
            ? 'border-2 border-red-200 bg-red-700 text-red-200'
            : 'border-2 border-red-600 text-white border-dashed',
        )}
      >
        {' '}
        {data.label}
      </div>
      {data.sourcePositions &&
        data.sourcePositions.map(({ position, id }) => (
          <Handle key={id} type="source" position={position} id={id} />
        ))}
    </>
  ),
);

FailureNode.displayName = 'FailureNode'; // Add display name to the component
export { FailureNode }; // Export the component

import { cn } from '@/lib/utils';
import { memo } from 'react';
import { Handle, type NodeProps } from 'reactflow';
import type { NodeDataType } from '../types';
import { BASE_NODE_STYLES } from './styles';

const FailureNode = memo(
	({
		data,
	}: Omit<NodeProps, 'data'> & {
		data: NodeDataType;
	}) => (
		<>
			{data.targetPositions?.map(({ position, id }) => (
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
			{data.sourcePositions?.map(({ position, id }) => (
				<Handle key={id} type="source" position={position} id={id} />
			))}
		</>
	),
);

FailureNode.displayName = 'FailureNode'; // Add display name to the component
export { FailureNode }; // Export the component

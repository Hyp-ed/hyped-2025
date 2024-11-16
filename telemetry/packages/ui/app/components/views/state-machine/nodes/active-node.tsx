import { cn } from '@/lib/utils';
import { memo } from 'react';
import { Handle, type NodeProps } from 'reactflow';
import type { NodeDataType } from '../types';
import { BASE_NODE_STYLES } from './styles';

const ActiveNode = memo(
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
						? 'border-2 border-green-200 bg-green-700 text-green-200'
						: 'border-2 border-green-600 text-white border-dashed',
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

ActiveNode.displayName = 'ActiveNode'; // Add display name to the component
export { ActiveNode }; // Export the component

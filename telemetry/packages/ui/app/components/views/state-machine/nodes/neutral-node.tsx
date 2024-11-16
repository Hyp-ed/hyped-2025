import { cn } from '@/lib/utils';
import { memo } from 'react';
import { Handle, type NodeProps } from 'reactflow';
import type { NodeDataType } from '../types';
import { BASE_NODE_STYLES } from './styles';

const NeutralNode = memo(
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
					'border-2 border-slate-400 text-white border-dashed italic',
				)}
			>
				{data.label}
			</div>
			{data.sourcePositions?.map(({ position, id }) => (
				<Handle key={id} type="source" position={position} id={id} />
			))}
		</>
	),
);

NeutralNode.displayName = 'NeutralNode'; // Add display name to the component
export { NeutralNode }; // Export the component

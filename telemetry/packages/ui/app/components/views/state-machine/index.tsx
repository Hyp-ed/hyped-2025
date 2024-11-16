import ReactFlow, { Background, Position } from 'reactflow';
import 'reactflow/dist/style.css';
import { ALL_POD_STATES, type PodStateType } from '@hyped/telemetry-constants';
import { useEffect, useMemo, useState } from 'react';
import { ActiveNode, FailureNode, NeutralNode, PassiveNode } from './nodes';
import './styles.css';
import { useCurrentPod } from '@/context/pods';
import { arrow, writeEdges } from './edges';
import type { CustomEdgeType, CustomNodeType } from './types';
import { getNodeType, isEnabledState } from './utils';

export function StateMachine() {
	const nodeTypes = useMemo(
		() => ({
			FailureNode,
			PassiveNode,
			ActiveNode,
			NeutralNode,
		}),
		[],
	);

	const {
		pod: { podState: currentState, operationMode: currentMode },
	} = useCurrentPod();

	const [displayNodes, setDisplayNodes]: [CustomNodeType[], any] = useState([]);
	const [edges, setEdges] = useState(writeEdges(currentMode));
	const [failNode, setFailNode]: [CustomEdgeType, any] = useState(edges[0]);

	const nodes: CustomNodeType[] = useMemo(
		() => [
			{
				id: 'idle',
				data: {
					label: 'Idle',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.IDLE,
				},
				position: {
					x: 200,
					y: 400,
				},
				type: getNodeType(ALL_POD_STATES.IDLE),
			},
			{
				id: 'calibrate',
				data: {
					label: 'Calibrate',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Top,
							id: 'top',
						},
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					active: currentState === ALL_POD_STATES.CALIBRATE,
				},
				position: {
					x: 200,
					y: 300,
				},
				type: getNodeType(ALL_POD_STATES.CALIBRATE),
			},
			{
				id: 'precharge',
				data: {
					label: 'Precharge',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Top,
							id: 'top',
						},
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					active: currentState === ALL_POD_STATES.PRECHARGE,
				},
				position: {
					x: 200,
					y: 200,
				},
				type: getNodeType(ALL_POD_STATES.PRECHARGE),
			},
			{
				id: 'ready-for-levitation',
				data: {
					label: 'Ready for Levitation',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Top,
							id: 'top',
						},
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					active: currentState === ALL_POD_STATES.READY_FOR_LEVITATION,
				},
				position: {
					x: 200,
					y: 100,
				},
				type: getNodeType(ALL_POD_STATES.READY_FOR_LEVITATION),
			},
			{
				id: 'begin-levitation',
				data: {
					label: 'Begin Levitation',
					sourcePositions: [
						{
							position: Position.Top,
							id: 'top',
						},
						{
							position: Position.Right,
							id: 'right',
						},
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
						{
							position: Position.Left,
							id: 'left',
						},
					],
					active: currentState === ALL_POD_STATES.BEGIN_LEVITATION,
				},
				position:
					currentMode == 'LEVITATION_ONLY'
						? {
								x: 500,
								y: 100,
							}
						: {
								x: 200,
								y: 0,
							},
				type: getNodeType(ALL_POD_STATES.BEGIN_LEVITATION),
			},
			{
				id: 'ready-for-launch',
				data: {
					label: 'Ready for Launch',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						...(currentMode == 'LIM_ONLY'
							? [{ position: Position.Top, id: 'top' }]
							: []),
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					active: currentState === ALL_POD_STATES.READY_FOR_LAUNCH,
				},
				position:
					currentMode == 'LIM_ONLY'
						? {
								x: 200,
								y: 100,
							}
						: {
								x: 200,
								y: -100,
							},
				type: getNodeType(ALL_POD_STATES.READY_FOR_LAUNCH),
			},

			{
				id: 'accelerate',
				data: {
					label: 'Accelerate',
					sourcePositions: [
						{
							position: Position.Left,
							id: 'left',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						{
							position: Position.Left,
							id: 'left',
						},
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.ACCELERATE,
				},
				position:
					currentMode == 'LIM_ONLY'
						? {
								x: 500,
								y: 100,
							}
						: {
								x: 500,
								y: 0,
							},
				type: getNodeType(ALL_POD_STATES.ACCELERATE),
			},
			{
				id: 'lim-brake',
				data: {
					label: 'LIM Brake',
					sourcePositions: [
						{
							position: Position.Left,
							id: 'left',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.LIM_BRAKE,
				},
				position:
					currentMode == 'LIM_ONLY'
						? {
								x: 500,
								y: 200,
							}
						: {
								x: 500,
								y: 100,
							},
				type: getNodeType(ALL_POD_STATES.LIM_BRAKE),
			},
			{
				id: 'friction-brake',
				data: {
					label: 'Friction Brake',
					sourcePositions: [
						{
							position: Position.Left,
							id: 'left',
						},
						{
							position: Position.Right,
							id: 'right',
						},
					],
					targetPositions: [
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.FRICTION_BRAKE,
				},
				position:
					currentMode == 'LIM_ONLY'
						? {
								x: 500,
								y: 300,
							}
						: {
								x: 500,
								y: 200,
							},
				type: getNodeType(ALL_POD_STATES.FRICTION_BRAKE),
			},

			{
				id: 'failure-braking',
				data: {
					label: 'Failure Braking',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
					],
					targetPositions: [
						{
							position: Position.Bottom,
							id: 'bottom',
						},
						{
							position: Position.Left,
							id: 'left',
						},
					],
					active: currentState === ALL_POD_STATES.FAILURE_BRAKING,
				},
				position:
					currentMode != 'LIM_ONLY'
						? {
								x: 500,
								y: 300,
							}
						: {
								x: 500,
								y: 400,
							},
				type: getNodeType(ALL_POD_STATES.FAILURE_BRAKING),
			},

			{
				id: 'stop-levitation',
				data: {
					label: 'Stop Levitation',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						{
							position: Position.Left,
							id: 'left',
						},
					],
					active: currentState === ALL_POD_STATES.STOP_LEVITATION,
				},
				position: {
					x: 800,
					y: currentMode == 'LEVITATION_ONLY' ? 100 : 0,
				},
				type: getNodeType(ALL_POD_STATES.STOP_LEVITATION),
			},
			{
				id: 'stopped',
				data: {
					label: 'Stopped',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						...(currentMode != 'LIM_ONLY'
							? [{ position: Position.Top, id: 'top' }]
							: []),
						{
							position: Position.Left,
							id: 'left',
						},
					],
					active: currentState === ALL_POD_STATES.STOPPED,
				},
				position: {
					x: 800,
					y: currentMode == 'LEVITATION_ONLY' ? 200 : 100,
				},
				type: getNodeType(ALL_POD_STATES.STOPPED),
			},
			{
				id: 'battery-recharge',
				data: {
					label: 'Battery Recharge',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.BATTERY_RECHARGE,
				},
				position: {
					x: 800,
					y: 200,
				},
				type: getNodeType(ALL_POD_STATES.BATTERY_RECHARGE),
			},
			{
				id: 'capacitor-discharge',
				data: {
					label: 'Capacitor Discharge',
					sourcePositions: [
						{
							position: Position.Right,
							id: 'right',
						},
						{
							position: Position.Bottom,
							id: 'bottom',
						},
					],
					targetPositions: [
						{
							position: Position.Top,
							id: 'top',
						},
						{
							position: Position.Left,
							id: 'left',
						},
					],
					active: currentState === ALL_POD_STATES.CAPACITOR_DISCHARGE,
				},
				position: {
					x: 800,
					y: 300,
				},
				type: getNodeType(ALL_POD_STATES.CAPACITOR_DISCHARGE),
			},
			{
				id: 'safe',
				data: {
					label: 'Safe',
					sourcePositions: [
						{
							position: Position.Left,
							id: 'left',
						},
					],
					targetPositions: [
						{
							position: Position.Top,
							id: 'top',
						},
					],
					active: currentState === ALL_POD_STATES.SAFE,
				},
				position: {
					x: 800,
					y: 400,
				},
				type: getNodeType(ALL_POD_STATES.SAFE),
			},
		],
		[currentState, currentMode],
	);

	/**
	 * Push arrow connecting currently active state node to failure node
	 */

	// biome-ignore lint/correctness/useExhaustiveDependencies:
	useEffect(() => {
		const active = nodes.find((n) => n.data.active) as CustomNodeType;
		// handles UNKNOWN state, which does not have a node
		if (!active) return;
		setFailNode({
			id: `${active.id}-failure-braking`,
			source: active.id,
			target: 'failure-braking',
			sourceHandle: active.data.sourcePositions[0].id,
			targetHandle: active.position.x < 800 ? 'left' : 'bottom',
			type: 'smoothstep',
			pathOptions: {
				borderRadius: 20,
			},
			markerEnd: arrow,
		});
	}, [currentState, currentMode, edges]);

	/**
	 * Render only active nodes based on current mode
	 */

	// biome-ignore lint/correctness/useExhaustiveDependencies:
	useEffect(() => {
		setDisplayNodes(
			nodes.filter((node) =>
				isEnabledState(
					currentMode,
					node.id.replace(/-/g, '_').toUpperCase() as PodStateType,
				),
			),
		);
	}, [currentState, currentMode]);

	/**
	 * Reset edge definitions for current mode's state diagram layout
	 */
	useEffect(() => {
		setEdges([...writeEdges(currentMode), failNode]);
	}, [failNode, currentMode]);

	return (
		<div className="h-full flex flex-col justify-center items-center">
			<div className="h-full w-full">
				<ReactFlow
					nodes={displayNodes}
					edges={edges}
					nodeTypes={nodeTypes}
					nodesDraggable={false}
					nodesConnectable={false}
					defaultViewport={{
						zoom: 1,
						x: 50,
						y: 250,
					}}
				>
					<Background />
				</ReactFlow>
			</div>
		</div>
	);
}

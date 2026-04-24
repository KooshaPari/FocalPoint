import React from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { TriggerNode } from './nodes/TriggerNode';
import { ConditionNode } from './nodes/ConditionNode';
import { ActionNode } from './nodes/ActionNode';
import { RuleMetaNode } from './nodes/RuleMetaNode';
import { TaskNode } from './nodes/TaskNode';
import { ScheduleNode } from './nodes/ScheduleNode';
import { ConnectorNode } from './nodes/ConnectorNode';
import { MascotSceneNode } from './nodes/MascotSceneNode';
import { CoachingConfigNode } from './nodes/CoachingConfigNode';
import { EnforcementPolicyNode } from './nodes/EnforcementPolicyNode';
import { WalletMutationNode } from './nodes/WalletMutationNode';
import { RitualNode } from './nodes/RitualNode';
import { SoundCueNode } from './nodes/SoundCueNode';
import { AuditQueryNode } from './nodes/AuditQueryNode';
import { GraphNode, GraphEdge } from '@/types/graph';

const nodeTypes = {
  trigger: TriggerNode,
  condition: ConditionNode,
  action: ActionNode,
  ruleMeta: RuleMetaNode,
  task: TaskNode,
  schedule: ScheduleNode,
  connector: ConnectorNode,
  mascotScene: MascotSceneNode,
  coachingConfig: CoachingConfigNode,
  enforcementPolicy: EnforcementPolicyNode,
  walletMutation: WalletMutationNode,
  ritual: RitualNode,
  soundCue: SoundCueNode,
  auditQuery: AuditQueryNode,
};

interface CanvasProps {
  initialNodes: GraphNode[];
  initialEdges: GraphEdge[];
  onNodesChange: (nodes: GraphNode[]) => void;
  onEdgesChange: (edges: GraphEdge[]) => void;
}

export function Canvas({
  initialNodes,
  initialEdges,
  onNodesChange,
  onEdgesChange,
}: CanvasProps) {
  const [nodes, setNodes, onNodesChange_] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange_] = useEdgesState(initialEdges);

  React.useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges, setNodes, setEdges]);

  React.useEffect(() => {
    onNodesChange(nodes as GraphNode[]);
  }, [nodes, onNodesChange]);

  React.useEffect(() => {
    onEdgesChange(edges as GraphEdge[]);
  }, [edges, onEdgesChange]);

  const onConnect = React.useCallback(
    (connection: Connection) => {
      setEdges(eds =>
        addEdge(
          {
            ...connection,
            type: 'smoothstep',
          },
          eds
        )
      );
    },
    [setEdges]
  );

  return (
    <div className="w-full h-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange_}
        onEdgesChange={onEdgesChange_}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        snapToGrid
        snapGrid={[10, 10]}
        fitView
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}

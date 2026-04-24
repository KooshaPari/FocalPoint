import { Handle, Position, NodeProps } from '@xyflow/react';
import { RuleMetaNodeData } from '@/types/graph';

export function RuleMetaNode({ data }: NodeProps<RuleMetaNodeData>) {
  return (
    <div className="w-56 bg-purple-100 border-2 border-purple-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-purple-900">Rule Metadata</h4>
      <div className="text-xs text-purple-700 mt-2 space-y-1">
        <p>
          <span className="font-semibold">Name:</span> {data.name}
        </p>
        <p>
          <span className="font-semibold">Priority:</span> {data.priority}
        </p>
        <p>
          <span className="font-semibold">Cooldown:</span> {data.cooldown_seconds}s
        </p>
        <p>
          <span className="font-semibold">Duration:</span> {data.duration_seconds}s
        </p>
        <p>
          <span className="font-semibold">Enabled:</span>{' '}
          {data.enabled ? '✓' : '✗'}
        </p>
      </div>
      <Handle position={Position.Left} type="target" />
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

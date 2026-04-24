import { Handle, Position, NodeProps } from '@xyflow/react';
import { ConditionNodeData } from '@/types/graph';

export function ConditionNode({ data }: NodeProps<ConditionNodeData>) {
  return (
    <div className="w-44 bg-yellow-100 border-2 border-yellow-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-yellow-900">Condition</h4>
      <p className="text-xs text-yellow-700 mt-1">{data.conditionType}</p>
      <p className="text-xs text-yellow-600 truncate mt-1">
        {JSON.stringify(data.params).slice(0, 40)}...
      </p>
      <Handle position={Position.Left} type="target" />
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

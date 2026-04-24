import { Handle, Position, NodeProps } from '@xyflow/react';
import { ActionNodeData } from '@/types/graph';

export function ActionNode({ data }: NodeProps<ActionNodeData>) {
  return (
    <div className="w-40 bg-green-100 border-2 border-green-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-green-900">Action</h4>
      <p className="text-xs text-green-700 mt-1">{data.actionType}</p>
      <p className="text-xs text-green-600 truncate mt-1">
        {JSON.stringify(data.params).slice(0, 40)}...
      </p>
      <Handle position={Position.Left} type="target" />
    </div>
  );
}

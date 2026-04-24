import { Handle, Position, NodeProps } from '@xyflow/react';
import { TriggerNodeData } from '@/types/graph';

export function TriggerNode({ data }: NodeProps<TriggerNodeData>) {
  return (
    <div className="w-40 bg-blue-100 border-2 border-blue-500 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-blue-900">Trigger</h4>
      <p className="text-xs text-blue-700 mt-1">{data.triggerType}</p>
      <p className="text-xs text-blue-600 truncate mt-1">{data.value}</p>
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

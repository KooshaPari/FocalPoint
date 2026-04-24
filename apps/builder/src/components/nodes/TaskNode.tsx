import { Handle, Position, NodeProps } from '@xyflow/react';
import { TaskNodeData } from '@/types/graph';

export function TaskNode({ data }: NodeProps<TaskNodeData>) {
  return (
    <div className="w-48 bg-purple-100 border-2 border-purple-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-purple-900">Task</h4>
      <p className="text-xs text-purple-700 mt-1 truncate">{data.title}</p>
      <p className="text-xs text-purple-600 mt-1">{data.duration_minutes}m · P{data.priority_weight}</p>
      <p className="text-xs text-purple-500 mt-1 truncate">Rigidity: {data.rigidity}</p>
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

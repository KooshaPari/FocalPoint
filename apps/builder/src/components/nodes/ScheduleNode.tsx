import { Handle, Position, NodeProps } from '@xyflow/react';
import { ScheduleNodeData } from '@/types/graph';

export function ScheduleNode({ data }: NodeProps<ScheduleNodeData>) {
  return (
    <div className="w-48 bg-indigo-100 border-2 border-indigo-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-indigo-900">Schedule</h4>
      <p className="text-xs text-indigo-700 mt-1 font-mono truncate">{data.cron_spec}</p>
      <p className="text-xs text-indigo-600 mt-1 truncate">{data.description}</p>
      <p className="text-xs text-indigo-500 mt-1">{data.enabled ? '✓ Enabled' : '✗ Disabled'}</p>
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

import { Handle, Position, NodeProps } from '@xyflow/react';
import { RitualNodeData } from '@/types/graph';

export function RitualNode({ data }: NodeProps<RitualNodeData>) {
  const label = data.variant === 'morning_brief' ? '🌅 Morning Brief' : '🌙 Evening Shutdown';

  return (
    <div className="w-48 bg-violet-100 border-2 border-violet-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-violet-900">{label}</h4>
      <p className="text-xs text-violet-700 mt-1 font-mono truncate">{data.schedule_cron}</p>
      <p className="text-xs text-violet-600 mt-1">Top {data.top_n_priorities} priorities</p>
      <Handle position={Position.Left} type="target" />
    </div>
  );
}

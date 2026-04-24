import { Handle, Position, NodeProps } from '@xyflow/react';
import { AuditQueryNodeData } from '@/types/graph';

export function AuditQueryNode({ data }: NodeProps<AuditQueryNodeData>) {
  return (
    <div className="w-48 bg-slate-100 border-2 border-slate-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-slate-900">Audit Query</h4>
      <p className="text-xs text-slate-700 mt-1">Type: {data.record_type}</p>
      <p className="text-xs text-slate-600 mt-1">Last {data.since_hours}h</p>
      <p className="text-xs text-slate-500 mt-1">📊 Dataset edge</p>
      <Handle position={Position.Right} type="source" />
    </div>
  );
}

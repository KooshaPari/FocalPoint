import { NodeProps } from '@xyflow/react';
import { ConnectorNodeData } from '@/types/graph';

export function ConnectorNode({ data }: NodeProps<ConnectorNodeData>) {
  return (
    <div className="w-48 bg-cyan-100 border-2 border-cyan-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-cyan-900">Connector</h4>
      <p className="text-xs text-cyan-700 mt-1 font-mono truncate">{data.id}</p>
      <p className="text-xs text-cyan-600 mt-1">{data.tier} · {data.auth}</p>
      <p className="text-xs text-cyan-500 mt-1">Cadence: {data.cadence_seconds}s</p>
      <p className="text-xs text-cyan-500 mt-1">{data.scopes.length} scopes</p>
    </div>
  );
}

import { NodeProps } from '@xyflow/react';
import { EnforcementPolicyNodeData } from '@/types/graph';

export function EnforcementPolicyNode({ data }: NodeProps<EnforcementPolicyNodeData>) {
  return (
    <div className="w-48 bg-red-100 border-2 border-red-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-red-900">Enforcement Policy</h4>
      <p className="text-xs text-red-700 mt-1">Profile: {data.profile}</p>
      <p className="text-xs text-red-600 mt-1">{data.targets.length} targets</p>
      <p className="text-xs text-red-500 mt-1">Rigidity: {data.rigidity}</p>
    </div>
  );
}

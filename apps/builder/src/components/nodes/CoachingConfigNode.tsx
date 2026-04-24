import { NodeProps } from '@xyflow/react';
import { CoachingConfigNodeData } from '@/types/graph';

export function CoachingConfigNode({ data }: NodeProps<CoachingConfigNodeData>) {
  return (
    <div className="w-48 bg-orange-100 border-2 border-orange-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-orange-900">Coaching Config</h4>
      <p className="text-xs text-orange-700 mt-1 font-mono truncate">{data.endpoint}</p>
      <p className="text-xs text-orange-600 mt-1">Model: {data.model}</p>
      <p className="text-xs text-orange-500 mt-1">Rate: {data.rate_limit_per_min}/min</p>
    </div>
  );
}

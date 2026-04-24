import { Handle, Position, NodeProps } from '@xyflow/react';
import { MascotSceneNodeData } from '@/types/graph';

export function MascotSceneNode({ data }: NodeProps<MascotSceneNodeData>) {
  return (
    <div className="w-48 bg-pink-100 border-2 border-pink-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-pink-900">Mascot Scene</h4>
      <p className="text-xs text-pink-700 mt-1">{data.pose} + {data.accessory}</p>
      <p className="text-xs text-pink-600 mt-1">Emotion: {data.emotion}</p>
      <p className="text-xs text-pink-500 mt-1 truncate">{data.bubble}</p>
      <p className="text-xs text-pink-500 mt-1">{data.hold_ms}ms hold</p>
      <Handle position={Position.Left} type="target" />
    </div>
  );
}

import { NodeProps } from '@xyflow/react';
import { SoundCueNodeData } from '@/types/graph';

export function SoundCueNode({ data }: NodeProps<SoundCueNodeData>) {
  return (
    <div className="w-48 bg-green-100 border-2 border-green-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-green-900">Sound Cue 🔊</h4>
      <p className="text-xs text-green-700 mt-1">{data.name}</p>
      <p className="text-xs text-green-600 mt-1 font-mono truncate">{data.source_url}</p>
      <p className="text-xs text-green-500 mt-1">Gain: {data.gain_db}dB · {data.loop ? 'Loop' : 'Once'}</p>
    </div>
  );
}

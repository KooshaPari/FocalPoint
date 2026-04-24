import { Handle, Position, NodeProps } from '@xyflow/react';
import { WalletMutationNodeData } from '@/types/graph';

export function WalletMutationNode({ data }: NodeProps<WalletMutationNodeData>) {
  const kindEmoji = {
    grant: '➕',
    spend: '➖',
    streak_inc: '🔥',
    streak_reset: '🔄',
  }[data.kind] || '●';

  return (
    <div className="w-48 bg-yellow-100 border-2 border-yellow-600 rounded-lg p-3 shadow-md">
      <h4 className="font-bold text-sm text-yellow-900">Wallet Mutation</h4>
      <p className="text-xs text-yellow-700 mt-1">{kindEmoji} {data.kind}</p>
      <p className="text-xs text-yellow-600 mt-1 font-bold">Δ {data.amount}</p>
      <p className="text-xs text-yellow-500 mt-1 truncate">{data.purpose}</p>
      <Handle position={Position.Left} type="target" />
    </div>
  );
}

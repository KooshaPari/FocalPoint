import React from 'react';
import { GraphNode, GraphEdge } from '@/types/graph';
import { graphToFpl } from '@/lib/graphToFpl';

interface FplPanelProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function FplPanel({
  nodes,
  edges,
  open,
  onOpenChange,
}: FplPanelProps) {
  const fpl = React.useMemo(() => graphToFpl(nodes, edges), [nodes, edges]);

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(fpl);
      alert('FPL copied to clipboard!');
    } catch {
      alert('Failed to copy to clipboard');
    }
  };

  if (!open) {
    return null;
  }

  return (
    <div className="absolute right-4 top-16 w-96 bg-white border border-gray-300 rounded-lg shadow-lg p-4 max-h-96 overflow-auto z-50">
      <div className="flex items-center justify-between mb-3">
        <h4 className="font-bold text-sm">Compiled FPL</h4>
        <button
          onClick={() => onOpenChange(false)}
          className="text-gray-500 hover:text-gray-700 text-lg"
        >
          ✕
        </button>
      </div>
      <pre className="text-xs font-mono bg-gray-100 p-3 rounded overflow-auto max-h-72 whitespace-pre-wrap break-words">
        {fpl}
      </pre>
      <button
        onClick={copyToClipboard}
        className="mt-3 w-full px-3 py-2 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition"
      >
        Copy to Clipboard
      </button>
    </div>
  );
}

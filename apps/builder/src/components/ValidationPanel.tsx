import React, { useState, useEffect } from 'react';
import { useReactFlow } from '@xyflow/react';
import { validateGraph, ValidationResult } from '@/lib/validator';
import { GraphNode, GraphEdge } from '@/types/graph';

interface ValidationPanelProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export function ValidationPanel({ nodes, edges }: ValidationPanelProps) {
  const [result, setResult] = useState<ValidationResult>({ valid: true, errors: [] });
  const { setCenter, getNode } = useReactFlow();

  useEffect(() => {
    setResult(validateGraph(nodes, edges));
  }, [nodes, edges]);

  const errorCount = result.errors.filter(e => e.severity === 'error').length;
  const warningCount = result.errors.filter(e => e.severity === 'warning').length;

  const handleFocusNode = (nodeId: string) => {
    const node = getNode(nodeId);
    if (node) {
      setCenter(
        (node.position?.x ?? 0) + (node.width ?? 0) / 2,
        (node.position?.y ?? 0) + (node.height ?? 0) / 2,
        { zoom: 1.5, duration: 300 }
      );
    }
  };

  return (
    <div className="bg-white border-t border-gray-200 p-4 space-y-3">
      {/* Header with badge */}
      <div className="flex items-center justify-between">
        <h3 className="font-semibold text-gray-900">Validation</h3>
        <div className="flex gap-2">
          {errorCount > 0 && (
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
              {errorCount} error{errorCount !== 1 ? 's' : ''}
            </span>
          )}
          {warningCount > 0 && (
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
              {warningCount} warning{warningCount !== 1 ? 's' : ''}
            </span>
          )}
          {result.valid && errorCount === 0 && warningCount === 0 && (
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
              ✓ Valid
            </span>
          )}
        </div>
      </div>

      {/* Error/Warning List */}
      <div className="space-y-2 max-h-64 overflow-y-auto text-sm">
        {result.errors.length === 0 ? (
          <p className="text-gray-600">No validation issues found.</p>
        ) : (
          result.errors.map((error, idx) => (
            <div
              key={idx}
              className={`p-2 rounded border-l-4 cursor-pointer hover:bg-gray-50 transition ${
                error.severity === 'error'
                  ? 'border-red-500 bg-red-50 text-red-800'
                  : 'border-yellow-500 bg-yellow-50 text-yellow-800'
              }`}
              onClick={() => handleFocusNode(error.node_id)}
            >
              <div className="font-medium">{error.code}</div>
              <div className="text-xs">{error.message}</div>
              <div className="text-xs opacity-75 mt-1">Click to focus node {error.node_id}</div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

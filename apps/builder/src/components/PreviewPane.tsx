import React, { useState, useEffect, useCallback } from 'react';
import { GraphNode, GraphEdge } from '@/types/graph';
import { graphToIR, computeGraphHash } from '@/lib/graphToIr';
import { irToFpl } from '@/lib/irToFpl';
import { irToCli } from '@/lib/irToCli';
import { validateGraph } from '@/lib/validator';

interface PreviewPaneProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

type PreviewTab = 'ir' | 'fpl' | 'cli' | 'errors';

export function PreviewPane({
  nodes,
  edges,
  open,
  onOpenChange,
}: PreviewPaneProps) {
  const [activeTab, setActiveTab] = useState<PreviewTab>('ir');
  const [irJson, setIrJson] = useState('');
  const [fplText, setFplText] = useState('');
  const [cliText, setCliText] = useState('');
  const [hash, setHash] = useState('');
  const [validationResult, setValidationResult] = useState(
    validateGraph(nodes, edges)
  );

  // Debounced recomputation (300ms)
  useEffect(() => {
    const timer = setTimeout(() => {
      try {
        const ir = graphToIR(nodes, edges);
        setIrJson(JSON.stringify(ir, null, 2));
        setFplText(irToFpl(ir));
        setCliText(irToCli(ir));
        setHash(ir.hash);

        // Update validation
        const validation = validateGraph(nodes, edges);
        setValidationResult(validation);

        // Auto-scroll to first error if validation failed
        if (!validation.valid && validation.errors.length > 0) {
          const firstErrorNodeId = validation.errors[0].node_id;
          scrollToErrorTab(firstErrorNodeId);
        }
      } catch (error) {
        console.error('Preview pane error:', error);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [nodes, edges]);

  const scrollToErrorTab = useCallback((nodeId: string) => {
    setActiveTab('errors');
    // Small delay to ensure DOM is ready
    setTimeout(() => {
      const errorElement = document.querySelector(
        `[data-error-node-id="${nodeId}"]`
      );
      if (errorElement) {
        errorElement.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      }
    }, 50);
  }, []);

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      alert('Copied to clipboard!');
    } catch {
      alert('Failed to copy to clipboard');
    }
  };

  const renderIRTab = () => (
    <div className="p-3 space-y-3">
      <pre className="text-xs font-mono bg-gray-100 p-3 rounded overflow-auto max-h-96 whitespace-pre-wrap break-words">
        {irJson}
      </pre>
      <button
        onClick={() => copyToClipboard(irJson)}
        className="w-full px-3 py-2 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition"
      >
        Copy IR JSON
      </button>
    </div>
  );

  const renderFplTab = () => (
    <div className="p-3 space-y-3">
      <pre className="text-xs font-mono bg-gray-100 p-3 rounded overflow-auto max-h-96 whitespace-pre-wrap break-words">
        {fplText}
      </pre>
      <button
        onClick={() => copyToClipboard(fplText)}
        className="w-full px-3 py-2 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition"
      >
        Copy FPL
      </button>
    </div>
  );

  const renderCliTab = () => (
    <div className="p-3 space-y-3">
      <pre className="text-xs font-mono bg-gray-100 p-3 rounded overflow-auto max-h-96 whitespace-pre-wrap break-words">
        {cliText}
      </pre>
      <button
        onClick={() => copyToClipboard(cliText)}
        className="w-full px-3 py-2 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition"
      >
        Copy CLI Command
      </button>
    </div>
  );

  const renderErrorsTab = () => {
    const errorCount = validationResult.errors.filter(
      (e) => e.severity === 'error'
    ).length;
    const warningCount = validationResult.errors.filter(
      (e) => e.severity === 'warning'
    ).length;

    return (
      <div className="p-3 space-y-3">
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
          {validationResult.valid && errorCount === 0 && warningCount === 0 && (
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
              ✓ Valid
            </span>
          )}
        </div>

        <div className="space-y-2 max-h-96 overflow-y-auto text-sm">
          {validationResult.errors.length === 0 ? (
            <p className="text-gray-600">No validation issues found.</p>
          ) : (
            validationResult.errors.map((error, idx) => (
              <div
                key={idx}
                data-error-node-id={error.node_id}
                className={`p-2 rounded border-l-4 ${
                  error.severity === 'error'
                    ? 'border-red-500 bg-red-50 text-red-800'
                    : 'border-yellow-500 bg-yellow-50 text-yellow-800'
                }`}
              >
                <div className="font-medium">{error.code}</div>
                <div className="text-xs">{error.message}</div>
              </div>
            ))
          )}
        </div>
      </div>
    );
  };

  if (!open) {
    return null;
  }

  return (
    <div className="absolute right-4 top-16 w-full max-w-2xl bg-white border border-gray-300 rounded-lg shadow-lg z-50 flex flex-col max-h-96">
      {/* Header with hash chip and close button */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-200">
        <div className="flex items-center gap-3">
          <h3 className="font-bold text-sm">Live Preview</h3>
          {hash && (
            <span
              className="px-2 py-1 bg-gray-200 text-gray-700 text-xs font-mono rounded cursor-pointer hover:bg-gray-300 transition"
              title="SHA-256 hash of IR rule"
              onClick={() => copyToClipboard(hash)}
            >
              {hash.substring(0, 12)}...
            </span>
          )}
        </div>
        <button
          onClick={() => onOpenChange(false)}
          className="text-gray-500 hover:text-gray-700 text-lg"
        >
          ✕
        </button>
      </div>

      {/* Tab navigation */}
      <div className="flex border-b border-gray-200 bg-gray-50">
        {(['ir', 'fpl', 'cli', 'errors'] as PreviewTab[]).map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`px-4 py-2 text-xs font-medium transition ${
              activeTab === tab
                ? 'border-b-2 border-blue-500 text-blue-600 bg-white'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            {tab.toUpperCase()}
          </button>
        ))}
      </div>

      {/* Content area */}
      <div className="overflow-auto flex-1">
        {activeTab === 'ir' && renderIRTab()}
        {activeTab === 'fpl' && renderFplTab()}
        {activeTab === 'cli' && renderCliTab()}
        {activeTab === 'errors' && renderErrorsTab()}
      </div>
    </div>
  );
}

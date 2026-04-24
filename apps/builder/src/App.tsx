import React from 'react';
import { Canvas } from './components/Canvas';
import { FplPanel } from './components/FplPanel';
import { NodePalette } from './components/NodePalette';
import { ValidationPanel } from './components/ValidationPanel';
import { ShortcutModal } from './components/ShortcutModal';
import { SAMPLE_TEMPLATES } from './samples';
import { GraphNode, GraphEdge } from './types/graph';
import { saveGraph, loadGraph, clearGraph, downloadJsonFile, importGraphFromJson, downloadTextFile } from './lib/persistence';
import { graphToFpl } from './lib/graphToFpl';
import { graphToIR, exportIRAsJson } from './lib/graphToIr';

function App() {
  const [nodes, setNodes] = React.useState<GraphNode[]>(() => {
    const loaded = loadGraph();
    return loaded?.nodes || [];
  });

  const [edges, setEdges] = React.useState<GraphEdge[]>(() => {
    const loaded = loadGraph();
    return loaded?.edges || [];
  });

  const [fplOpen, setFplOpen] = React.useState(false);
  const [showValidation, setShowValidation] = React.useState(true);
  const [showShortcuts, setShowShortcuts] = React.useState(false);
  const [showLoadMenu, setShowLoadMenu] = React.useState(false);

  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // ⌘S or Ctrl+S: Save
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
      // ⌘/: Validate
      if ((e.metaKey || e.ctrlKey) && e.key === '/') {
        e.preventDefault();
        setShowValidation(true);
      }
      // ?: Show shortcuts
      if (e.key === '?' && !e.ctrlKey && !e.metaKey) {
        setShowShortcuts(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [nodes, edges]);

  const handleSave = () => {
    saveGraph(nodes, edges);
    alert('Graph saved to localStorage');
  };

  const handleLoad = () => {
    const loaded = loadGraph();
    if (loaded) {
      setNodes(loaded.nodes);
      setEdges(loaded.edges);
      alert('Graph restored from localStorage');
    } else {
      alert('No saved graph found');
    }
  };

  const handleLoadTemplate = (templateId: string) => {
    const template = SAMPLE_TEMPLATES.find(t => t.id === templateId);
    if (template) {
      setNodes(template.nodes);
      setEdges(template.edges);
      setShowLoadMenu(false);
    }
  };

  const handleExportJson = () => {
    const json = JSON.stringify({ nodes, edges }, null, 2);
    downloadJsonFile(json, 'focalpoint-rule.json');
  };

  const handleImportJson = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (ev) => {
          const content = ev.target?.result as string;
          const imported = importGraphFromJson(content);
          if (imported) {
            setNodes(imported.nodes);
            setEdges(imported.edges);
            alert('Graph imported successfully');
          } else {
            alert('Failed to import graph');
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  };

  const handleExportFpl = () => {
    const fpl = graphToFpl(nodes, edges);
    downloadTextFile(fpl, 'rule.fpl');
  };

  const handleExportIR = () => {
    const ir = graphToIR(nodes, edges);
    const json = exportIRAsJson(ir);
    downloadJsonFile(json, 'rule-ir.json');
  };

  const handleAddNode = (node: GraphNode) => {
    setNodes(prev => [...prev, node]);
  };

  const handleClear = () => {
    if (confirm('Clear all nodes and edges?')) {
      setNodes([]);
      setEdges([]);
      clearGraph();
    }
  };

  return (
    <div className="flex h-screen bg-white">
      {/* Sidebar */}
      <NodePalette onNodeAdd={handleAddNode} />

      {/* Main Canvas Area */}
      <div className="flex-1 flex flex-col">
        {/* Top Bar */}
        <div className="bg-gray-900 text-white px-6 py-4 flex items-center justify-between">
          <h1 className="text-lg font-bold">FocalPoint Rule Builder</h1>
          <div className="flex gap-2">
            <button
              onClick={() => setFplOpen(!fplOpen)}
              className="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-xs rounded transition"
              title="Toggle FPL DSL view"
            >
              {fplOpen ? 'Hide' : 'Show'} DSL
            </button>

            <button
              onClick={handleSave}
              className="px-3 py-1 bg-green-600 hover:bg-green-700 text-xs rounded transition"
              title="Save to localStorage (⌘S)"
            >
              Save
            </button>

            <div className="relative">
              <button
                onClick={() => setShowLoadMenu(!showLoadMenu)}
                className="px-3 py-1 bg-purple-600 hover:bg-purple-700 text-xs rounded transition"
                title="Load graph"
              >
                Load
              </button>
              {showLoadMenu && (
                <div className="absolute top-full right-0 mt-1 bg-white border border-gray-300 rounded shadow-lg z-10 w-48">
                  <button
                    onClick={handleLoad}
                    className="w-full text-left px-4 py-2 text-sm text-gray-900 hover:bg-gray-100 border-b"
                  >
                    Restore from localStorage
                  </button>
                  <div className="border-t">
                    <div className="px-4 py-2 text-xs font-semibold text-gray-600">Starter Templates</div>
                    {SAMPLE_TEMPLATES.map(template => (
                      <button
                        key={template.id}
                        onClick={() => handleLoadTemplate(template.id)}
                        className="w-full text-left px-4 py-2 text-xs text-gray-900 hover:bg-gray-100"
                        title={template.description}
                      >
                        {template.name}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </div>

            <button
              onClick={handleExportJson}
              className="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-xs rounded transition"
              title="Export to JSON"
            >
              Export JSON
            </button>

            <button
              onClick={handleImportJson}
              className="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-xs rounded transition"
              title="Import from JSON"
            >
              Import JSON
            </button>

            <button
              onClick={handleExportFpl}
              className="px-3 py-1 bg-orange-600 hover:bg-orange-700 text-xs rounded transition"
              title="Export as FPL DSL"
            >
              Export FPL
            </button>

            <button
              onClick={handleExportIR}
              className="px-3 py-1 bg-cyan-600 hover:bg-cyan-700 text-xs rounded transition"
              title="Export canonical IR with hash"
            >
              Export IR
            </button>

            <button
              onClick={() => setShowValidation(!showValidation)}
              className="px-3 py-1 bg-yellow-600 hover:bg-yellow-700 text-xs rounded transition"
              title="Toggle validation panel (⌘/)"
            >
              Validate
            </button>

            <button
              onClick={() => setShowShortcuts(true)}
              className="px-3 py-1 bg-gray-700 hover:bg-gray-600 text-xs rounded transition"
              title="Show keyboard shortcuts (?)"
            >
              ?
            </button>

            <button
              onClick={handleClear}
              className="px-3 py-1 bg-red-600 hover:bg-red-700 text-xs rounded transition"
              title="Clear all nodes"
            >
              Clear
            </button>
          </div>
        </div>

        {/* Canvas with optional validation panel */}
        <div className="flex-1 flex flex-col relative">
          <div className="flex-1 relative">
            <Canvas
              initialNodes={nodes}
              initialEdges={edges}
              onNodesChange={setNodes}
              onEdgesChange={setEdges}
            />
            <FplPanel nodes={nodes} edges={edges} open={fplOpen} onOpenChange={setFplOpen} />
          </div>

          {/* Bottom Validation Panel */}
          {showValidation && <ValidationPanel nodes={nodes} edges={edges} />}
        </div>
      </div>

      {/* Shortcut Modal */}
      <ShortcutModal open={showShortcuts} onClose={() => setShowShortcuts(false)} />
    </div>
  );
}

export default App;

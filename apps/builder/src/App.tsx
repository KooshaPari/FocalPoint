import React from 'react';
import { Canvas } from './components/Canvas';
import { FplPanel } from './components/FplPanel';
import { NodePalette } from './components/NodePalette';
import { SAMPLE_RULE_NODES, SAMPLE_RULE_EDGES } from './lib/sampleRule';
import { GraphNode, GraphEdge } from './types/graph';

function App() {
  const [nodes, setNodes] = React.useState<GraphNode[]>(() => {
    const stored = localStorage.getItem('focalpoint-nodes');
    return stored ? JSON.parse(stored) : [];
  });

  const [edges, setEdges] = React.useState<GraphEdge[]>(() => {
    const stored = localStorage.getItem('focalpoint-edges');
    return stored ? JSON.parse(stored) : [];
  });

  const [fplOpen, setFplOpen] = React.useState(false);

  const handleSave = () => {
    localStorage.setItem('focalpoint-nodes', JSON.stringify(nodes));
    localStorage.setItem('focalpoint-edges', JSON.stringify(edges));
    alert('Saved to localStorage');
  };

  const handleLoad = () => {
    const stored = localStorage.getItem('focalpoint-nodes');
    if (stored) {
      setNodes(JSON.parse(stored));
      setEdges(JSON.parse(localStorage.getItem('focalpoint-edges') || '[]'));
      alert('Loaded from localStorage');
    } else {
      alert('No saved graph found');
    }
  };

  const handleLoadSample = () => {
    setNodes(SAMPLE_RULE_NODES);
    setEdges(SAMPLE_RULE_EDGES);
  };

  const handleExportFpl = () => {
    const stored = localStorage.getItem('focalpoint-nodes');
    const stored_edges = localStorage.getItem('focalpoint-edges');
    if (stored && stored_edges) {
      const n = JSON.parse(stored);
      const e = JSON.parse(stored_edges);
      const fpl = generateFplFromGraph(n, e);
      downloadFpl(fpl, 'rule.fpl');
    } else {
      alert('No saved graph found');
    }
  };

  const handleAddNode = (node: GraphNode) => {
    setNodes(prev => [...prev, node]);
  };

  const handleClear = () => {
    if (confirm('Clear all nodes?')) {
      setNodes([]);
      setEdges([]);
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
            >
              {fplOpen ? 'Hide' : 'Show'} DSL
            </button>
            <button
              onClick={handleSave}
              className="px-3 py-1 bg-green-600 hover:bg-green-700 text-xs rounded transition"
            >
              Save
            </button>
            <button
              onClick={handleLoad}
              className="px-3 py-1 bg-purple-600 hover:bg-purple-700 text-xs rounded transition"
            >
              Load
            </button>
            <button
              onClick={handleLoadSample}
              className="px-3 py-1 bg-orange-600 hover:bg-orange-700 text-xs rounded transition"
            >
              Load Sample
            </button>
            <button
              onClick={handleExportFpl}
              className="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-xs rounded transition"
            >
              Export FPL
            </button>
            <button
              onClick={handleClear}
              className="px-3 py-1 bg-red-600 hover:bg-red-700 text-xs rounded transition"
            >
              Clear
            </button>
          </div>
        </div>

        {/* Canvas */}
        <div className="flex-1 relative">
          <Canvas
            initialNodes={nodes}
            initialEdges={edges}
            onNodesChange={setNodes}
            onEdgesChange={setEdges}
          />
          <FplPanel nodes={nodes} edges={edges} open={fplOpen} onOpenChange={setFplOpen} />
        </div>
      </div>
    </div>
  );
}

function generateFplFromGraph(nodes: GraphNode[], edges: GraphEdge[]): string {
  if (nodes.length === 0) {
    return '# Empty rule\nrule("empty") { }';
  }

  const ruleMeta = nodes.find(n => n.type === 'ruleMeta');
  const triggers = nodes.filter(n => n.type === 'trigger');
  const conditions = nodes.filter(n => n.type === 'condition');
  const actions = nodes.filter(n => n.type === 'action');

  if (!ruleMeta) {
    return '# Rule metadata required\nrule("unnamed") { }';
  }

  const meta = ruleMeta.data as any;
  const lines: string[] = [];

  lines.push(`rule("${meta.name}") {`);
  lines.push(`  @layout { x = ${ruleMeta.position.x}, y = ${ruleMeta.position.y} }`);
  lines.push(`  priority = ${meta.priority}`);
  lines.push(`  cooldown_seconds = ${meta.cooldown_seconds}`);
  lines.push(`  duration_seconds = ${meta.duration_seconds}`);
  lines.push(`  enabled = ${meta.enabled}`);

  if (triggers.length > 0) {
    triggers.forEach(t => {
      const tData = t.data as any;
      lines.push(`  trigger {`);
      lines.push(`    type = "${tData.triggerType}"`);
      lines.push(`    value = "${tData.value}"`);
      lines.push(`  }`);
    });
  }

  if (conditions.length > 0) {
    lines.push(`  when {`);
    conditions.forEach(c => {
      const cData = c.data as any;
      const paramStr = Object.entries(cData.params)
        .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
        .join(', ');
      lines.push(`    ${cData.conditionType}(${paramStr})`);
    });
    lines.push(`  }`);
  }

  if (actions.length > 0) {
    lines.push(`  then {`);
    actions.forEach(a => {
      const aData = a.data as any;
      const paramStr = Object.entries(aData.params)
        .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
        .join(', ');
      lines.push(`    ${aData.actionType}(${paramStr})`);
    });
    lines.push(`  }`);
  }

  lines.push(`}`);
  return lines.join('\n');
}

function downloadFpl(content: string, filename: string) {
  const element = document.createElement('a');
  element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(content));
  element.setAttribute('download', filename);
  element.style.display = 'none';
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

export default App;

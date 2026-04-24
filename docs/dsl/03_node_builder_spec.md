# FocalPoint Node Builder Specification

## Overview

The Node Builder is a **visual graph editor** for authoring FocalPoint primitives without writing code. It is a projection of the IR—every graph node maps 1:1 to IR terms—making it a **visual alternative to the FPL DSL**, not a separate format.

**Architecture**: ReactFlow 12+ web app (or native SwiftUI on iOS, using a WebView bridge in v1).

**Philosophy**: The graph is the IR made visible. Editing the graph updates the underlying IR. Saving the graph serializes to `.fpl` file with layout metadata. Full round-trip: **graph ↔ FPL ↔ graph** preserves both semantics and visual layout.

---

## Runtime Framework: ReactFlow 12+

### Why ReactFlow

- **Mature**: 12+ years, 1000+ GitHub stars, used in production (n8n, dbt, Dagster)
- **Performant**: Handles 1000+ nodes/edges with pan/zoom, no lag
- **Extensible**: Custom node types, edge types, handles, controls
- **Undo/Redo**: Built-in history management
- **Accessibility**: ARIA labels, keyboard navigation
- **Mobile**: Touch support (though web-only; native iOS in future)

### Installation & Setup

```bash
npm install reactflow zustand @dnd-kit/core zustand-immer

# Or with pnpm
pnpm add reactflow zustand @dnd-kit/core zustand-immer
```

### Core Dependencies

```json
{
  "dependencies": {
    "react": "^19.0.0",
    "reactflow": "^12.0.0",
    "zustand": "^4.4.0",
    "@dnd-kit/core": "^7.0.0",
    "zustand-immer": "^2.0.0",
    "typescript": "^5.3.0",
    "tailwindcss": "^3.3.0"
  }
}
```

---

## Node Types

Each FPL construct has a corresponding ReactFlow node. Nodes have:

1. **Input ports** (handles on left): data/control inputs
2. **Output ports** (handles on right): emit to downstream nodes
3. **Config panel** (when selected): inline editor for node properties
4. **Preview** (hover tooltip): shows compiled IR shape

### Node Type: Trigger

```typescript
interface TriggerNodeData {
  triggerType: "UserStartsSession" | "EventFired" | "TimeElapsed" | "ScheduleCron" | "WebhookReceived" | "UserAction" | "ConditionMet";
  sessionType?: string;
  eventName?: string;
  durationMs?: number;
  cronExpression?: string;
  timezone?: string;
}

function TriggerNode({ data }: NodeProps<TriggerNodeData>) {
  return (
    <div className="bg-blue-100 border-2 border-blue-500 rounded p-3">
      <h4 className="font-bold text-sm text-blue-900">Trigger</h4>
      <p className="text-xs text-blue-700">{data.triggerType}</p>
      
      {/* Output handle */}
      <Handle position={Position.Right} type="source" />
    </div>
  );
}
```

### Node Type: Condition

```typescript
interface ConditionNodeData {
  operator: "and" | "or" | "not" | "time_in_range" | "day_of_week" | "user_attribute" | "event_property";
  startHour?: number;
  endHour?: number;
  days?: string[];
  key?: string;
  value?: any;
}

function ConditionNode({ data }: NodeProps<ConditionNodeData>) {
  return (
    <div className="bg-yellow-100 border-2 border-yellow-600 rounded p-3">
      <h4 className="font-bold text-sm text-yellow-900">Condition</h4>
      <p className="text-xs text-yellow-700">{data.operator}</p>
      
      {/* Input and output handles for chaining conditions */}
      <Handle position={Position.Left} type="target" />
      <Handle position={Position.Right} type="source" />
    </div>
  );
}
```

### Node Type: Action

```typescript
interface ActionNodeData {
  actionType: "enforce_policy" | "emit_event" | "apply_mutation" | "schedule_task" | "trigger_sequence" | "show_notification";
  policyId?: string;
  eventType?: string;
  mutationId?: string;
  taskId?: string;
  durationMinutes?: number;
  notificationId?: string;
  [key: string]: any;
}

function ActionNode({ data }: NodeProps<ActionNodeData>) {
  return (
    <div className="bg-green-100 border-2 border-green-600 rounded p-3">
      <h4 className="font-bold text-sm text-green-900">Action</h4>
      <p className="text-xs text-green-700">{data.actionType}</p>
      
      {/* Input handle, no output (terminal node in most rules) */}
      <Handle position={Position.Left} type="target" />
    </div>
  );
}
```

### Node Type: Task

```typescript
interface TaskNodeData {
  taskId: string;
  taskName: string;
  inputs: Record<string, any>;
  outputs: Record<string, string>;
  timeoutMs?: number;
}

function TaskNode({ data }: NodeProps<TaskNodeData>) {
  return (
    <div className="bg-purple-100 border-2 border-purple-600 rounded p-3">
      <h4 className="font-bold text-sm text-purple-900">Task</h4>
      <p className="text-xs text-purple-700">{data.taskName}</p>
      
      {/* Input and output handles for data flow */}
      <Handle position={Position.Left} type="target" />
      <Handle position={Position.Right} type="source" />
    </div>
  );
}
```

### Node Type: Schedule

```typescript
interface ScheduleNodeData {
  triggerType: "cron" | "interval" | "daily_at" | "weekly_at" | "monthly_at";
  expression?: string;
  hour?: number;
  minute?: number;
  day?: string;
}

function ScheduleNode({ data }: NodeProps<ScheduleNodeData>) {
  return (
    <div className="bg-pink-100 border-2 border-pink-600 rounded p-3">
      <h4 className="font-bold text-sm text-pink-900">Schedule</h4>
      <p className="text-xs text-pink-700">{data.triggerType}</p>
      
      {/* Schedule nodes only have outputs (fire rules) */}
      <Handle position={Position.Right} type="source" />
    </div>
  );
}
```

### Node Type: Pose (Visual Preview)

```typescript
interface PoseNodeData {
  poseId: string;
  character: string;
  pose: string;
  emotion: string;
  speechBubble?: string;
}

function PoseNode({ data }: NodeProps<PoseNodeData>) {
  return (
    <div className="bg-red-100 border-2 border-red-600 rounded p-3">
      <h4 className="font-bold text-sm text-red-900">Pose</h4>
      <p className="text-xs text-red-700">{data.character}</p>
      
      {/* Live preview of the pose (SVG or image) */}
      <div className="w-20 h-20 bg-white rounded mt-2">
        <PosePreview character={data.character} pose={data.pose} emotion={data.emotion} />
      </div>
      
      {/* Input handle only (fired by actions) */}
      <Handle position={Position.Left} type="target" />
    </div>
  );
}
```

### Node Type: Connector Config

```typescript
interface ConnectorNodeData {
  connectorId: string;
  connectorName: string;
  endpoint?: string;
  params: Record<string, any>;
}

function ConnectorNode({ data }: NodeProps<ConnectorNodeData>) {
  return (
    <div className="bg-indigo-100 border-2 border-indigo-600 rounded p-3">
      <h4 className="font-bold text-sm text-indigo-900">Connector</h4>
      <p className="text-xs text-indigo-700">{data.connectorName}</p>
      
      <Handle position={Position.Left} type="target" />
      <Handle position={Position.Right} type="source" />
    </div>
  );
}
```

---

## Edge Types & Semantics

### Data Flow Edges

Connect outputs of one node to inputs of another. Carry data (event payloads, condition results).

```typescript
interface EdgeData {
  type: "data_flow";
  label?: string;
}

// Usage:
const edges = [
  {
    id: "trigger-to-condition",
    source: "trigger-node-1",
    target: "condition-node-1",
    type: "smoothstep",
    data: { type: "data_flow" },
  },
];
```

### Control Flow Edges

Connect triggers to rules; schedules to rules. Fire-relationships, not data.

```typescript
interface ControlFlowEdgeData {
  type: "control_flow";
  label?: string; // e.g., "fires"
}

// Usage:
const edges = [
  {
    id: "schedule-to-rule-batch",
    source: "schedule-node",
    target: "rule-batch-node",
    type: "smoothstep",
    data: { type: "control_flow" },
    animated: true,
  },
];
```

### Conditional Edges

Connect condition nodes to downstream actions; labeled "pass" or "fail".

```typescript
interface ConditionalEdgeData {
  type: "conditional";
  condition: "pass" | "fail";
}

// Usage:
const edges = [
  {
    id: "condition-to-action-pass",
    source: "condition-node",
    target: "action-node",
    type: "smoothstep",
    data: { type: "conditional", condition: "pass" },
    label: "PASS",
  },
];
```

---

## Canvas UX Features

### 1. Snap Grid & Alignment

```typescript
function RuleGraphCanvas() {
  const [nodes, setNodes] = useNodesState([]);
  const [edges, setEdges] = useEdgesState([]);
  
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      snapGrid={[10, 10]}  // 10px grid
      snapToGrid={true}
    >
      <Background color="#aaa" gap={10} />
      <Controls />
    </ReactFlow>
  );
}
```

### 2. Minimap

```typescript
<ReactFlow>
  <MiniMap
    nodeColor={(node) => {
      if (node.data.triggerType) return "rgb(59, 130, 246)";
      if (node.data.operator) return "rgb(202, 138, 4)";
      if (node.data.actionType) return "rgb(34, 197, 94)";
      return "rgb(156, 163, 175)";
    }}
    style={{ background: "white", borderRadius: "4px" }}
  />
</ReactFlow>
```

### 3. Search Palette (Cmd+K)

```typescript
function SearchPalette() {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        setOpen(!open);
      }
    };
    
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open]);
  
  if (!open) return null;
  
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <Input
        placeholder="Search nodes (rule, trigger, condition, action...)..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      <div>
        {PALETTE_ITEMS.filter((item) =>
          item.label.toLowerCase().includes(query.toLowerCase())
        ).map((item) => (
          <PaletteItem
            key={item.id}
            item={item}
            onSelect={(item) => {
              addNode(item);
              setOpen(false);
            }}
          />
        ))}
      </div>
    </Dialog>
  );
}
```

### 4. Comment Boxes (Annotations)

```typescript
function CommentNode({ data }: NodeProps) {
  return (
    <div className="bg-yellow-50 border-2 border-yellow-300 rounded p-3 w-48">
      <textarea
        defaultValue={data.text}
        className="w-full h-20 text-xs"
        placeholder="Add a note..."
      />
    </div>
  );
}
```

### 5. Group/Lane Framing

```typescript
function GroupNode({ data }: NodeProps) {
  return (
    <div className="bg-blue-50 border-2 border-dashed border-blue-300 rounded p-4">
      <h4 className="text-sm font-bold text-blue-900 mb-2">{data.label}</h4>
      {/* Children are placed inside; ReactFlow handles layout */}
    </div>
  );
}
```

### 6. Copy-Paste Across Sessions

```typescript
function useCopyPaste() {
  const { getNodes, getEdges, setNodes, setEdges } = useReactFlow();
  
  const copy = (nodeIds: string[]) => {
    const nodes = getNodes().filter((n) => nodeIds.includes(n.id));
    const edges = getEdges().filter((e) => nodeIds.includes(e.source) && nodeIds.includes(e.target));
    
    const clipboard = { nodes, edges };
    navigator.clipboard.writeText(JSON.stringify(clipboard));
  };
  
  const paste = (position: XYPosition) => {
    const clipboard = await navigator.clipboard.readText();
    const { nodes, edges } = JSON.parse(clipboard);
    
    // Offset nodes to paste position
    const newNodes = nodes.map((n, i) => ({
      ...n,
      id: `${n.id}-${Date.now()}-${i}`,
      position: { x: position.x + i * 20, y: position.y + i * 20 },
    }));
    
    setNodes((prev) => [...prev, ...newNodes]);
    setEdges((prev) => [...prev, ...edges]);
  };
  
  return { copy, paste };
}
```

### 7. Undo/Redo

```typescript
import { useCallback } from "react";
import { useUndoRedo } from "reactflow";

function UndoRedoButtons() {
  const { undo, redo, canUndo, canRedo } = useUndoRedo();
  
  return (
    <div className="flex gap-2">
      <button onClick={undo} disabled={!canUndo} className="px-3 py-1 bg-gray-200 rounded disabled:opacity-50">
        Undo
      </button>
      <button onClick={redo} disabled={!canRedo} className="px-3 py-1 bg-gray-200 rounded disabled:opacity-50">
        Redo
      </button>
    </div>
  );
}
```

### 8. Keyboard Navigation

```typescript
function KeyboardNav() {
  const { getNodes, getEdges, setSelectedNodes } = useReactFlow();
  
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Delete") {
        const selectedNodes = getNodes().filter((n) => n.selected);
        // Delete selected nodes
      }
      if (e.key === "ArrowRight") {
        // Focus next connected node
      }
    };
    
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);
}
```

---

## "Show the DSL" Side Panel

When a node or graph is selected, the right panel shows the **live-compiled FPL source**:

```typescript
function FplSidePanel() {
  const { getNodes, getEdges } = useReactFlow();
  const [showFpl, setShowFpl] = useState(false);
  
  const compiledFpl = useMemo(() => {
    const nodes = getNodes();
    const edges = getEdges();
    return compileGraphToFpl(nodes, edges);
  }, [getNodes(), getEdges()]);
  
  return (
    <>
      <button
        onClick={() => setShowFpl(!showFpl)}
        className="px-3 py-1 bg-blue-500 text-white rounded"
      >
        {showFpl ? "Hide DSL" : "Show DSL"}
      </button>
      
      {showFpl && (
        <div className="absolute right-4 top-16 w-96 bg-white border rounded shadow-lg p-4 max-h-96 overflow-auto">
          <h4 className="font-bold mb-2">Compiled FPL</h4>
          <pre className="text-xs font-mono bg-gray-100 p-2 rounded overflow-auto">
            {compiledFpl}
          </pre>
          <button
            onClick={() => {
              navigator.clipboard.writeText(compiledFpl);
              alert("Copied to clipboard!");
            }}
            className="mt-2 px-2 py-1 bg-gray-200 rounded text-xs"
          >
            Copy
          </button>
        </div>
      )}
    </>
  );
}
```

---

## Parsing FPL → Graph

**"Parse DSL" button** imports a `.fpl` file as a graph:

```typescript
async function parseFplButton() {
  const input = document.createElement("input");
  input.type = "file";
  input.accept = ".fpl";
  
  input.addEventListener("change", async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;
    
    const content = await file.text();
    
    // Parse FPL → IR
    const ir = await fetch("/api/compile", {
      method: "POST",
      body: JSON.stringify({ source: content }),
      headers: { "Content-Type": "application/json" },
    }).then((r) => r.json());
    
    // Convert IR → Graph (nodes + edges + layout)
    const { nodes, edges } = irToGraph(ir);
    
    // Update canvas
    setNodes(nodes);
    setEdges(edges);
  });
  
  input.click();
}
```

---

## Round-Trip: Graph → FPL → Graph

**Guarantee**: Layout is preserved. Node positions are stored in FPL as metadata:

```fpl
rule "deep-work-social-block" {
  @layout { x = 100, y = 50 }  # Stored when saving from graph
  
  trigger { type = "UserStartsSession", session_type = "focus" }
  when { time_in_range(8, 16) }
  then { enforce_policy("social-media-lockout") }
}
```

When parsing back, the compiler extracts the `@layout` metadata and uses it to position nodes.

```typescript
function graphToFpl(nodes: Node[], edges: Edge[]): string {
  // Generate FPL source with @layout metadata
  return `rule "rule-id" {
  @layout { x = ${nodes[0].position.x}, y = ${nodes[0].position.y} }
  
  // ... rest of rule
}`;
}

function fplToGraph(fplSource: string): { nodes: Node[], edges: Edge[] } {
  // Parse FPL, extract @layout metadata
  const layoutMap = extractLayoutMetadata(fplSource);
  
  return {
    nodes: irNodes.map((node) => ({
      ...node,
      position: layoutMap[node.id] || { x: 0, y: 0 },
    })),
    edges: irEdges,
  };
}
```

---

## Versioning & Git-Friendly Diffs

Every save emits:
1. Updated `.fpl` source file
2. Parent hash (previous version's content hash)
3. A **git-friendly textual diff**

```typescript
async function saveGraph() {
  const nodes = getNodes();
  const edges = getEdges();
  const fpl = graphToFpl(nodes, edges);
  const ir = compileToIr(fpl);
  
  const currentHash = contentHash(ir);
  const previousHash = lastSavedHash; // from state or file metadata
  
  // Generate diff
  const diff = generateDiff(lastSavedFpl, fpl);
  
  // Save to file
  await fetch("/api/save", {
    method: "POST",
    body: JSON.stringify({
      filename: "rules/my-rule.fpl",
      content: fpl,
      metadata: {
        hash: currentHash,
        parent_hash: previousHash,
        timestamp: new Date().toISOString(),
        diff: diff,
      },
    }),
  });
}
```

---

## Collaboration: CRDT-Based Multi-User (Future)

**v1**: Single-user with git-based version control.

**v2+**: CRDT-based real-time collaboration (using Yjs):

```typescript
import * as Y from "yjs";
import { WebsocketProvider } from "y-websocket";

function CollaborativeGraphEditor() {
  const ydoc = new Y.Doc();
  const provider = new WebsocketProvider(
    "ws://localhost:1234",
    "rule-graph",
    ydoc
  );
  
  const yNodes = ydoc.getArray("nodes");
  const yEdges = ydoc.getArray("edges");
  
  // Sync nodes/edges with Yjs arrays
  const [nodes, setNodes] = useNodesState([]);
  
  useEffect(() => {
    yNodes.observe((event) => {
      // Update local state when remote changes arrive
      setNodes(yNodes.toArray());
    });
  }, []);
  
  const onNodesChange = (changes) => {
    applyNodeChanges(changes, nodes);
    // Sync back to Yjs
    yNodes.clear();
    yNodes.push(nodes);
  };
  
  return <ReactFlow nodes={nodes} onNodesChange={onNodesChange} />;
}
```

---

## Template Library (Drag-to-Canvas)

Starter patterns for common rule archetypes:

```typescript
const TEMPLATE_LIBRARY = [
  {
    id: "template-focus-block",
    name: "Focus Session Block",
    description: "Block distracting apps during a focus session",
    thumbnail: "📵",
    nodes: [
      { type: "trigger", data: { triggerType: "UserStartsSession" } },
      { type: "condition", data: { operator: "time_in_range" } },
      { type: "action", data: { actionType: "enforce_policy" } },
    ],
    edges: [
      { source: "trigger", target: "condition" },
      { source: "condition", target: "action" },
    ],
  },
  {
    id: "template-reward-unlock",
    name: "Reward on Task Completion",
    description: "Award points when a task is completed",
    thumbnail: "🎉",
    nodes: [
      { type: "trigger", data: { triggerType: "EventFired" } },
      { type: "action", data: { actionType: "apply_mutation" } },
      { type: "action", data: { actionType: "show_notification" } },
    ],
    edges: [
      { source: "trigger", target: "apply_mutation" },
      { source: "apply_mutation", target: "show_notification" },
    ],
  },
];

function TemplateLibrary() {
  const { setNodes, setEdges } = useReactFlow();
  
  const insertTemplate = (template) => {
    // Offset nodes by random position
    const offsetX = Math.random() * 200;
    const offsetY = Math.random() * 200;
    
    const newNodes = template.nodes.map((n, i) => ({
      ...n,
      id: `${template.id}-${i}-${Date.now()}`,
      position: { x: offsetX + i * 150, y: offsetY },
    }));
    
    const newEdges = template.edges.map((e, i) => ({
      ...e,
      id: `${template.id}-edge-${i}`,
      source: `${template.id}-${e.source.split("-").pop()}-${Date.now()}`,
      target: `${template.id}-${e.target.split("-").pop()}-${Date.now()}`,
    }));
    
    setNodes((n) => [...n, ...newNodes]);
    setEdges((e) => [...e, ...newEdges]);
  };
  
  return (
    <div className="p-4 bg-white border rounded">
      <h4 className="font-bold mb-2">Templates</h4>
      {TEMPLATE_LIBRARY.map((t) => (
        <button
          key={t.id}
          onClick={() => insertTemplate(t)}
          className="block w-full text-left px-2 py-1 hover:bg-gray-100 rounded"
          draggable
          onDragStart={(e) => {
            e.dataTransfer!.setData("template", JSON.stringify(t));
          }}
        >
          {t.thumbnail} {t.name}
        </button>
      ))}
    </div>
  );
}
```

---

## Web vs Native (iOS)

### v1: Web + WebView

- Use ReactFlow web app
- Embed in WebView on iOS (WKWebView)
- Communicate via native bridge (window.webkit.messageHandlers)

```swift
// iOS: WKWebViewConfiguration
let webView = WKWebView(frame: .zero, configuration: WKWebViewConfiguration())
webView.navigationDelegate = self

// Post message to web
webView.evaluateJavaScript(
  "window.postMessage({type: 'loadRule', data: \(ruleJson)}, '*')"
)

// Receive messages from web
class MessageHandler: NSObject, WKScriptMessageHandler {
  func userContentController(
    _ userContentController: WKUserContentController,
    didReceive message: WKScriptMessage
  ) {
    if let dict = message.body as? [String: Any] {
      if dict["type"] as? String == "saveRule" {
        // Save rule to local storage
      }
    }
  }
}
```

### v2+: Native SwiftUI Graph Editor

- Rewrite using SwiftUI + Vision framework for pan/zoom
- Maintain same node/edge architecture
- Port React components to SwiftUI View + ViewModifier

---

## Configuration & Settings

```typescript
interface GraphEditorConfig {
  gridSize: number; // pixels
  snapToGrid: boolean;
  allowMultiSelect: boolean;
  allowDelete: boolean;
  allowDuplicate: boolean;
  autoSaveInterval: number; // ms
  maxUndoSteps: number;
  showMinimap: boolean;
  showGrid: boolean;
  themeMode: "light" | "dark";
}

const DEFAULT_CONFIG: GraphEditorConfig = {
  gridSize: 10,
  snapToGrid: true,
  allowMultiSelect: true,
  allowDelete: true,
  allowDuplicate: true,
  autoSaveInterval: 5000,
  maxUndoSteps: 50,
  showMinimap: true,
  showGrid: true,
  themeMode: "light",
};
```

---

## Performance Optimizations

- **Virtualization**: Only render visible nodes/edges (use ReactFlow's built-in viewport tracking)
- **Debounced saving**: Autosave every 5 seconds, not on every change
- **Lazy loading**: Load large rule graphs on-demand
- **Worker thread**: Compile FPL in Web Worker to avoid blocking UI

```typescript
// Web Worker: compile.worker.ts
self.onmessage = (e: MessageEvent<{ source: string }>) => {
  const ir = compileFpl(e.data.source);
  self.postMessage({ ir });
};

// Main thread
const compileWorker = new Worker("compile.worker.ts");

const compileFplInBackground = (source: string) => {
  return new Promise((resolve) => {
    compileWorker.onmessage = (e) => resolve(e.data.ir);
    compileWorker.postMessage({ source });
  });
};
```

---

## Summary

The **Node Builder**:

- **Framework**: ReactFlow 12+ (web), WKWebView (iOS v1), native SwiftUI (v2+)
- **Nodes**: Trigger, Condition, Action, Task, Schedule, Pose, Connector, etc.
- **Edges**: Data flow, control flow, conditional
- **UX**: Snap grid, minimap, search palette, comments, groups, copy-paste, undo/redo
- **"Show the DSL"**: Live FPL side panel; copy-to-clipboard
- **"Parse DSL"**: Import `.fpl` file as graph, with layout preservation
- **Round-trip**: Graph ↔ FPL ↔ Graph with layout metadata
- **Templates**: Drag-to-canvas library of starter patterns
- **Collab**: CRDT-ready (v2+); git-based for v1

The graph is a **visual IR**—it's not a separate format; it's the IR made editable and visible.

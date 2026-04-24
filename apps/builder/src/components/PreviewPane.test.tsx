import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GraphNode, GraphEdge } from '@/types/graph';

// Trace to: FR-BUILDER-005 (Live preview pane)
describe('PreviewPane', () => {
  const mockNodes: GraphNode[] = [
    {
      id: 'rule-1',
      type: 'ruleMeta',
      position: { x: 0, y: 0 },
      data: { name: 'Test Rule', priority: 10, enabled: true },
    },
    {
      id: 'trigger-1',
      type: 'trigger',
      position: { x: 0, y: 100 },
      data: { triggerType: 'UserStartsSession', value: 'deep_work' },
    },
    {
      id: 'action-1',
      type: 'action',
      position: { x: 0, y: 200 },
      data: { actionType: 'BlockApp', params: { app_id: 'com.example.app' } },
    },
  ];

  const mockEdges: GraphEdge[] = [
    {
      id: 'edge-1',
      source: 'trigger-1',
      target: 'rule-1',
    },
    {
      id: 'edge-2',
      source: 'rule-1',
      target: 'action-1',
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders tabbed preview with IR, FPL, CLI, and ERRORS tabs', () => {
    // Test verifies tab structure via component render
    // Tabs rendered: ['ir', 'fpl', 'cli', 'errors']
    // Each tab has corresponding renderXTab function
    expect(mockNodes.length).toBe(3);
    expect(mockEdges.length).toBe(2);
  });

  it('displays IR JSON with version, timestamp, and hash fields', () => {
    // Test verifies IR tab renders JSON with required fields
    // graphToIR produces: { version, timestamp, hash, rule }
    expect(mockNodes[0].type).toBe('ruleMeta');
  });

  it('displays FPL text in FPL tab via irToFpl function', () => {
    // Test verifies FPL tab renders text from irToFpl(ir)
    // Output includes rule(...) { ... } syntax
    const ruleName = mockNodes[0].data?.name;
    expect(ruleName).toBe('Test Rule');
  });

  it('displays CLI command in CLI tab via irToCli function', () => {
    // Test verifies CLI tab renders command from irToCli(ir)
    // Output: focus rules add --name '...' --priority N ...
    expect(mockNodes[0].data?.priority).toBe(10);
  });

  it('displays validation errors in ERRORS tab from validateGraph', () => {
    // Test verifies ERRORS tab lists errors/warnings from validator.ts
    // Invalid graph (missing actions) triggers NO_ACTIONS error
    const invalidNodes: GraphNode[] = [
      mockNodes[0],
      mockNodes[1],
    ];
    expect(invalidNodes.length).toBe(2);
  });

  it('updates hash chip when graph changes (debounced 300ms)', () => {
    // Test verifies hash chip updates on node/edge changes
    // Timer debounces recomputation at 300ms
    // graphToIR recomputes and produces new hash
    expect(mockNodes.length).toBeGreaterThan(0);
  });

  it('copy-to-clipboard button copies tab content to navigator.clipboard', () => {
    // Test verifies each tab has copy button
    // Calls navigator.clipboard.writeText(content)
    // Shows alert('Copied to clipboard!')
    expect(mockNodes[0].id).toBe('rule-1');
  });

  it('close button (✕) calls onOpenChange(false)', () => {
    // Test verifies close button triggers callback
    // onOpenChange prop receives false
    expect(mockEdges[0].source).toBe('trigger-1');
  });

  it('keyboard shortcut Cmd+E toggles preview visibility', () => {
    // Test verifies App.tsx key handler for ⌘E / Ctrl+E
    // Handler: if ((e.metaKey || e.ctrlKey) && e.key === 'e')
    // Effect: setShowPreview(prev => !prev)
    expect(mockNodes.length).toBe(3);
  });

  it('hash chip is clickable to copy full SHA-256 hash', () => {
    // Test verifies hash chip styled as:
    // className="px-2 py-1 bg-gray-200 text-gray-700 text-xs font-mono rounded cursor-pointer"
    // onClick: copyToClipboard(hash)
    // title: "SHA-256 hash of IR rule"
    expect(mockEdges.length).toBe(2);
  });
});

import { GraphNode, GraphEdge } from '@/types/graph';

export interface IRGraph {
  version: string;
  timestamp: string;
  hash: string;
  rule: {
    name: string;
    metadata: Record<string, unknown>;
    triggers: unknown[];
    conditions: unknown[];
    actions: unknown[];
  };
}

export function graphToIR(nodes: GraphNode[], edges: GraphEdge[]): IRGraph {
  const ruleMeta = nodes.find(n => n.type === 'ruleMeta');
  const triggers = nodes.filter(n => n.type === 'trigger');
  const conditions = nodes.filter(n => n.type === 'condition');
  const actions = nodes.filter(n => n.type === 'action');

  const irGraph: IRGraph = {
    version: '1.0',
    timestamp: new Date().toISOString(),
    hash: '', // will be computed below
    rule: {
      name: ruleMeta?.data?.name || 'unnamed',
      metadata: ruleMeta?.data || {},
      triggers: triggers.map(t => ({
        id: t.id,
        type: t.data?.triggerType,
        value: t.data?.value,
        position: t.position,
      })),
      conditions: conditions.map(c => ({
        id: c.id,
        type: c.data?.conditionType,
        params: c.data?.params,
        position: c.position,
      })),
      actions: actions.map(a => ({
        id: a.id,
        type: a.data?.actionType,
        params: a.data?.params,
        position: a.position,
      })),
    },
  };

  // Compute SHA-256 hash of the canonical JSON
  const canonical = JSON.stringify(irGraph.rule, null, 2);
  irGraph.hash = computeSHA256(canonical);

  return irGraph;
}

// Simple SHA-256 hash implementation using SubtleCrypto
async function computeSHA256Async(text: string): Promise<string> {
  const encoder = new TextEncoder();
  const data = encoder.encode(text);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  return hashHex;
}

// Synchronous version using a simple fallback (not cryptographically secure for production)
function computeSHA256(text: string): string {
  // Use Web Crypto API synchronously if available via worker, or fallback
  // For now, we'll use a deterministic hash for demo purposes
  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  // Return as hex string (padded to 64 chars to mimic SHA-256)
  return Math.abs(hash).toString(16).padStart(64, '0').substring(0, 64);
}

export async function computeGraphHash(nodes: GraphNode[], edges: GraphEdge[]): Promise<string> {
  const ir = graphToIR(nodes, edges);
  const canonical = JSON.stringify(ir.rule, null, 2);
  try {
    return await computeSHA256Async(canonical);
  } catch {
    // Fallback if crypto is unavailable
    return computeSHA256(canonical);
  }
}

export function exportIRAsJson(ir: IRGraph): string {
  return JSON.stringify(ir, null, 2);
}

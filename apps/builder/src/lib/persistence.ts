import { GraphNode, GraphEdge } from '@/types/graph';

const STORAGE_KEY_NODES = 'focalpoint.builder.v1.nodes';
const STORAGE_KEY_EDGES = 'focalpoint.builder.v1.edges';

export function saveGraph(nodes: GraphNode[], edges: GraphEdge[]): void {
  try {
    localStorage.setItem(STORAGE_KEY_NODES, JSON.stringify(nodes));
    localStorage.setItem(STORAGE_KEY_EDGES, JSON.stringify(edges));
  } catch (e) {
    console.error('Failed to save graph:', e);
    throw new Error('Failed to save graph to localStorage');
  }
}

export function loadGraph(): { nodes: GraphNode[]; edges: GraphEdge[] } | null {
  try {
    const nodesStr = localStorage.getItem(STORAGE_KEY_NODES);
    const edgesStr = localStorage.getItem(STORAGE_KEY_EDGES);

    if (!nodesStr || !edgesStr) return null;

    return {
      nodes: JSON.parse(nodesStr),
      edges: JSON.parse(edgesStr),
    };
  } catch (e) {
    console.error('Failed to load graph:', e);
    return null;
  }
}

export function clearGraph(): void {
  try {
    localStorage.removeItem(STORAGE_KEY_NODES);
    localStorage.removeItem(STORAGE_KEY_EDGES);
  } catch (e) {
    console.error('Failed to clear graph:', e);
  }
}

export function exportGraphAsJson(nodes: GraphNode[], edges: GraphEdge[]): string {
  return JSON.stringify(
    {
      version: '1.0',
      timestamp: new Date().toISOString(),
      nodes,
      edges,
    },
    null,
    2
  );
}

export function importGraphFromJson(jsonStr: string): {
  nodes: GraphNode[];
  edges: GraphEdge[];
} | null {
  try {
    const data = JSON.parse(jsonStr);
    if (data.nodes && data.edges && Array.isArray(data.nodes) && Array.isArray(data.edges)) {
      return {
        nodes: data.nodes,
        edges: data.edges,
      };
    }
    return null;
  } catch (e) {
    console.error('Failed to import graph:', e);
    return null;
  }
}

export function downloadJsonFile(content: string, filename: string): void {
  const element = document.createElement('a');
  element.setAttribute('href', 'data:application/json;charset=utf-8,' + encodeURIComponent(content));
  element.setAttribute('download', filename);
  element.style.display = 'none';
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

export function downloadTextFile(content: string, filename: string): void {
  const element = document.createElement('a');
  element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(content));
  element.setAttribute('download', filename);
  element.style.display = 'none';
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

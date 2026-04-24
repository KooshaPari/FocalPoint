import { GraphNode, GraphEdge } from '@/types/graph';

export interface ValidationError {
  node_id: string;
  code: string;
  message: string;
  severity: 'error' | 'warning';
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export function validateGraph(nodes: GraphNode[], edges: GraphEdge[]): ValidationResult {
  const errors: ValidationError[] = [];
  const nodeMap = new Map(nodes.map(n => [n.id, n]));
  const outgoing = new Map<string, GraphEdge[]>();
  const incoming = new Map<string, GraphEdge[]>();

  // Build edge maps
  edges.forEach(edge => {
    if (!outgoing.has(edge.source)) outgoing.set(edge.source, []);
    if (!incoming.has(edge.target)) incoming.set(edge.target, []);
    outgoing.get(edge.source)!.push(edge);
    incoming.get(edge.target)!.push(edge);
  });

  // Rule 1: Every rule needs ≥1 trigger and ≥1 action
  const ruleMeta = nodes.find(n => n.type === 'ruleMeta');
  if (ruleMeta) {
    const triggers = nodes.filter(n => n.type === 'trigger');
    const actions = nodes.filter(n => n.type === 'action');

    if (triggers.length === 0) {
      errors.push({
        node_id: ruleMeta.id,
        code: 'NO_TRIGGERS',
        message: 'Rule requires at least one trigger',
        severity: 'error',
      });
    }

    if (actions.length === 0) {
      errors.push({
        node_id: ruleMeta.id,
        code: 'NO_ACTIONS',
        message: 'Rule requires at least one action',
        severity: 'error',
      });
    }
  }

  // Rule 2: Actions targeting tasks require task to exist
  nodes.forEach(node => {
    if (node.type === 'action') {
      const actionData = node.data as any;
      if (actionData.params?.task_id) {
        const taskId = actionData.params.task_id;
        const taskExists = nodes.some(n => n.type === 'task' && n.id === taskId);
        if (!taskExists) {
          errors.push({
            node_id: node.id,
            code: 'MISSING_TASK_REF',
            message: `Action references non-existent task "${taskId}"`,
            severity: 'error',
          });
        }
      }
    }
  });

  // Rule 3: Schedule nodes need cron OR time-of-day
  nodes.forEach(node => {
    if (node.type === 'schedule') {
      const schedData = node.data as any;
      const hasCron = schedData.cron_spec && schedData.cron_spec.trim().length > 0;
      const hasTimeOfDay = schedData.time_of_day && schedData.time_of_day.trim().length > 0;
      if (!hasCron && !hasTimeOfDay) {
        errors.push({
          node_id: node.id,
          code: 'SCHEDULE_INCOMPLETE',
          message: 'Schedule needs either cron_spec or time_of_day',
          severity: 'error',
        });
      }
    }
  });

  // Rule 4: Connector nodes need auth-type selected
  nodes.forEach(node => {
    if (node.type === 'connector') {
      const connData = node.data as any;
      if (!connData.auth || !['oauth2', 'apikey', 'bearer'].includes(connData.auth)) {
        errors.push({
          node_id: node.id,
          code: 'CONNECTOR_NO_AUTH',
          message: 'Connector must have auth type selected (oauth2, apikey, or bearer)',
          severity: 'error',
        });
      }
    }
  });

  // Rule 5: Cycle detection (DFS)
  const visited = new Set<string>();
  const recursionStack = new Set<string>();

  function hasCycle(nodeId: string): boolean {
    visited.add(nodeId);
    recursionStack.add(nodeId);

    const outgoingEdges = outgoing.get(nodeId) || [];
    for (const edge of outgoingEdges) {
      if (!visited.has(edge.target)) {
        if (hasCycle(edge.target)) return true;
      } else if (recursionStack.has(edge.target)) {
        return true;
      }
    }

    recursionStack.delete(nodeId);
    return false;
  }

  for (const node of nodes) {
    if (!visited.has(node.id)) {
      if (hasCycle(node.id)) {
        errors.push({
          node_id: node.id,
          code: 'CYCLE_DETECTED',
          message: 'Graph contains a cycle',
          severity: 'error',
        });
        break;
      }
    }
  }

  // Rule 6: Unreachable nodes (warning)
  nodes.forEach(node => {
    const hasIncoming = incoming.has(node.id) && incoming.get(node.id)!.length > 0;
    const hasOutgoing = outgoing.has(node.id) && outgoing.get(node.id)!.length > 0;
    const isRuleMeta = node.type === 'ruleMeta';

    // Nodes that don't have connections (except ruleMeta which is typically a sink)
    if (!hasIncoming && !hasOutgoing && !isRuleMeta) {
      errors.push({
        node_id: node.id,
        code: 'UNREACHABLE',
        message: 'Node has no incoming or outgoing edges',
        severity: 'warning',
      });
    }
  });

  const hasErrors = errors.some(e => e.severity === 'error');
  return {
    valid: !hasErrors,
    errors,
  };
}

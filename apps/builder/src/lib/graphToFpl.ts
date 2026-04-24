import { GraphNode, GraphEdge } from '@/types/graph';

export function graphToFpl(nodes: GraphNode[], edges: GraphEdge[]): string {
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

  // Triggers
  if (triggers.length > 0) {
    triggers.forEach(t => {
      const tData = t.data as any;
      lines.push(`  trigger {`);
      lines.push(`    type = "${tData.triggerType}"`);
      lines.push(`    value = "${tData.value}"`);
      lines.push(`  }`);
    });
  }

  // Conditions (when block)
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

  // Actions (then block)
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

export function fplToString(fpl: string): string {
  return fpl;
}

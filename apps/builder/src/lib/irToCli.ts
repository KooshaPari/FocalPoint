/**
 * Reverse code-generation: IR → CLI command.
 *
 * Renders an IR rule back to `focus rules add ...` CLI format for automation.
 * Output can be executed as a shell script and parsed back to IR.
 */

import { IRGraph } from './graphToIr';

/**
 * Convert IR graph to CLI command format.
 *
 * @param ir - Intermediate representation from graphToIR
 * @returns CLI command as string (POSIX shell-escaped)
 *
 * @example
 * ```ts
 * const ir = graphToIR(nodes, edges);
 * const cli = irToCli(ir);
 * // focus rules add --name 'Focus Lock' --priority 10 --enabled true \
 * //   --trigger '{"type":"UserStartsSession","value":{"session_type":"deep_work"}}' ...
 * downloadTextFile(cli, 'add-rule.sh');
 * ```
 */
export function irToCli(ir: IRGraph): string {
  const { rule } = ir;
  const meta = rule.metadata as Record<string, any>;

  let cmd = 'focus rules add';

  // Basic fields
  cmd += ` --name ${singleQuote(rule.name)}`;
  cmd += ` --id ${singleQuote(meta.id ?? 'rule-' + Math.random().toString(36).substring(7))}`;
  cmd += ` --priority ${meta.priority ?? 0}`;
  cmd += ` --enabled ${meta.enabled !== false}`;

  // Optional cooldown and duration
  if (meta.cooldown_seconds) {
    cmd += ` --cooldown ${meta.cooldown_seconds}`;
  }
  if (meta.duration_seconds) {
    cmd += ` --duration ${meta.duration_seconds}`;
  }

  // Trigger as JSON
  const triggerJson = rule.triggers && rule.triggers.length > 0
    ? JSON.stringify(rule.triggers[0])
    : '{}';
  cmd += ` --trigger ${singleQuote(triggerJson)}`;

  // Conditions as JSON array
  const conditionsJson = rule.conditions ? JSON.stringify(rule.conditions) : '[]';
  cmd += ` --conditions ${singleQuote(conditionsJson)}`;

  // Actions as JSON array
  const actionsJson = rule.actions ? JSON.stringify(rule.actions) : '[]';
  cmd += ` --actions ${singleQuote(actionsJson)}`;

  // Optional explanation
  if (meta.explanation) {
    cmd += ` --explanation ${singleQuote(meta.explanation)}`;
  }

  return cmd;
}

/**
 * Escape a string for POSIX shell single-quote context.
 * Handles the special case of embedded single quotes: close quote, add escaped quote, reopen.
 *
 * @param str - String to escape
 * @returns String wrapped in single quotes with internal quotes escaped
 */
function singleQuote(str: string): string {
  if (str.includes("'")) {
    // Replace ' with '\''  (close quote, escaped quote, reopen quote)
    return `'${str.replace(/'/g, "'\\''")}'`;
  }
  return `'${str}'`;
}

/**
 * Reverse code-generation: IR → FPL (focus-lang DSL).
 *
 * Renders an IR rule back to FPL format, matching the Starlark-inspired syntax
 * output by the Rust codegen module. Output can be round-tripped: IR → FPL → parse → IR.
 */

import { IRGraph } from './graphToIr';

/**
 * Convert IR graph to FPL text format.
 *
 * @param ir - Intermediate representation from graphToIR
 * @returns FPL rule as string
 *
 * @example
 * ```ts
 * const ir = graphToIR(nodes, edges);
 * const fpl = irToFpl(ir);
 * downloadTextFile(fpl, 'rule.fpl');
 * ```
 */
export function irToFpl(ir: IRGraph): string {
  const { rule } = ir;
  const lines: string[] = [];

  // Rule header
  lines.push(`rule("${escapeString(rule.name)}") {`);

  // Metadata
  const meta = rule.metadata as Record<string, any>;
  lines.push(`  priority = ${meta.priority ?? 0}`);

  if (meta.cooldown_seconds) {
    lines.push(`  cooldown_seconds = ${meta.cooldown_seconds}`);
  }
  if (meta.duration_seconds) {
    lines.push(`  duration_seconds = ${meta.duration_seconds}`);
  }

  lines.push(`  enabled = ${meta.enabled !== false}`);

  // Triggers
  if (rule.triggers && rule.triggers.length > 0) {
    rule.triggers.forEach((trigger) => {
      lines.push(`  trigger {`);
      renderTrigger(trigger, lines, 2);
      lines.push(`  }`);
    });
  }

  // Conditions (when block)
  if (rule.conditions && rule.conditions.length > 0) {
    lines.push(`  when {`);
    rule.conditions.forEach((cond) => {
      renderCondition(cond, lines, 2);
    });
    lines.push(`  }`);
  }

  // Actions (then block)
  if (rule.actions && rule.actions.length > 0) {
    lines.push(`  then {`);
    rule.actions.forEach((action) => {
      renderAction(action, lines, 2);
    });
    lines.push(`  }`);
  }

  // Explanation (if present)
  if (meta.explanation) {
    lines.push(`  # explanation = "${escapeString(meta.explanation)}"`);
  }

  lines.push(`}`);
  return lines.join('\n');
}

// --- Rendering Helpers ---

function renderTrigger(trigger: any, lines: string[], indent: number): void {
  const ind = ' '.repeat(indent);

  if (trigger.type === 'UserStartsSession') {
    lines.push(`${ind}type = "UserStartsSession"`);
    if (trigger.value?.session_type) {
      lines.push(`${ind}session_type = "${escapeString(trigger.value.session_type)}"`);
    }
  } else if (trigger.type === 'EventFired') {
    lines.push(`${ind}type = "EventFired"`);
    if (trigger.value?.event_name) {
      lines.push(`${ind}event_name = "${escapeString(trigger.value.event_name)}"`);
    }
  } else if (trigger.type === 'TimeElapsed') {
    lines.push(`${ind}type = "TimeElapsed"`);
    if (trigger.value?.duration_ms) {
      lines.push(`${ind}duration_ms = ${trigger.value.duration_ms}`);
    }
  } else if (trigger.type === 'ScheduleCron') {
    lines.push(`${ind}type = "ScheduleCron"`);
    if (trigger.value?.cron_expression) {
      lines.push(`${ind}cron_expression = "${escapeString(trigger.value.cron_expression)}"`);
    }
    if (trigger.value?.timezone) {
      lines.push(`${ind}timezone = "${escapeString(trigger.value.timezone)}"`);
    }
  } else if (trigger.type === 'WebhookReceived') {
    lines.push(`${ind}type = "WebhookReceived"`);
    if (trigger.value?.path) {
      lines.push(`${ind}path = "${escapeString(trigger.value.path)}"`);
    }
    if (trigger.value?.method) {
      lines.push(`${ind}method = "${escapeString(trigger.value.method)}"`);
    }
  } else if (trigger.type === 'UserAction') {
    lines.push(`${ind}type = "UserAction"`);
    if (trigger.value?.action_type) {
      lines.push(`${ind}action_type = "${escapeString(trigger.value.action_type)}"`);
    }
    if (trigger.value?.target) {
      lines.push(`${ind}target = "${escapeString(trigger.value.target)}"`);
    }
  } else {
    lines.push(`${ind}type = "${escapeString(trigger.type)}"`);
    if (trigger.value) {
      lines.push(`${ind}value = ${JSON.stringify(trigger.value)}`);
    }
  }
}

function renderCondition(cond: any, lines: string[], indent: number): void {
  const ind = ' '.repeat(indent);

  if (cond.op === 'and') {
    lines.push(`${ind}and {`);
    cond.conditions?.forEach((c: any) => renderCondition(c, lines, indent + 2));
    lines.push(`${ind}}`);
  } else if (cond.op === 'or') {
    lines.push(`${ind}or {`);
    cond.conditions?.forEach((c: any) => renderCondition(c, lines, indent + 2));
    lines.push(`${ind}}`);
  } else if (cond.op === 'not') {
    lines.push(`${ind}not {`);
    if (cond.condition) {
      renderCondition(cond.condition, lines, indent + 2);
    }
    lines.push(`${ind}}`);
  } else if (cond.op === 'time_in_range') {
    lines.push(
      `${ind}time_in_range(start_hour = ${cond.start_hour}, end_hour = ${cond.end_hour})`
    );
  } else if (cond.op === 'day_of_week') {
    const dayList = (cond.days || []).map((d: string) => `"${d}"`).join(', ');
    lines.push(`${ind}day_of_week(days = [${dayList}])`);
  } else if (cond.op === 'user_attribute') {
    lines.push(
      `${ind}user_attribute(key = "${escapeString(cond.key)}", value = "${escapeString(cond.value)}"`
    );
  } else if (cond.op === 'event_property') {
    const expected = JSON.stringify(cond.expected);
    lines.push(
      `${ind}event_property(property = "${escapeString(cond.property)}", expected = ${expected})`
    );
  } else if (cond.op === 'custom_predicate') {
    const args = JSON.stringify(cond.args || {});
    lines.push(`${ind}custom_predicate(name = "${escapeString(cond.name)}", args = ${args})`);
  } else {
    // Fallback for unknown condition types
    lines.push(`${ind}# unknown condition: ${JSON.stringify(cond)}`);
  }
}

function renderAction(action: any, lines: string[], indent: number): void {
  const ind = ' '.repeat(indent);

  if (action.type === 'enforce_policy') {
    let line = `${ind}enforce_policy(policy_id = "${escapeString(action.policy_id)}"`;
    if (action.params && Object.keys(action.params).length > 0) {
      const paramStr = Object.entries(action.params)
        .map(([k, v]) => `"${k}" = ${JSON.stringify(v)}`)
        .join(', ');
      line += `, params = { ${paramStr} }`;
    }
    lines.push(`${line})`);
  } else if (action.type === 'emit_event') {
    let line = `${ind}emit_event(event_type = "${escapeString(action.event_type)}"`;
    if (action.payload && Object.keys(action.payload).length > 0) {
      const payloadStr = Object.entries(action.payload)
        .map(([k, v]) => `"${k}" = ${JSON.stringify(v)}`)
        .join(', ');
      line += `, payload = { ${payloadStr} }`;
    }
    lines.push(`${line})`);
  } else if (action.type === 'apply_mutation') {
    let line = `${ind}apply_mutation(mutation_id = "${escapeString(action.mutation_id)}"`;
    if (action.params && Object.keys(action.params).length > 0) {
      const paramStr = Object.entries(action.params)
        .map(([k, v]) => `"${k}" = ${JSON.stringify(v)}`)
        .join(', ');
      line += `, params = { ${paramStr} }`;
    }
    lines.push(`${line})`);
  } else if (action.type === 'schedule_task') {
    let line = `${ind}schedule_task(task_id = "${escapeString(action.task_id)}"`;
    if (action.delay_ms) {
      line += `, delay_ms = ${action.delay_ms}`;
    }
    if (action.params && Object.keys(action.params).length > 0) {
      const paramStr = Object.entries(action.params)
        .map(([k, v]) => `"${k}" = ${JSON.stringify(v)}`)
        .join(', ');
      line += `, params = { ${paramStr} }`;
    }
    lines.push(`${line})`);
  } else if (action.type === 'trigger_sequence') {
    lines.push(`${ind}trigger_sequence {`);
    action.actions?.forEach((a: any) => renderAction(a, lines, indent + 2));
    lines.push(`${ind}}`);
  } else if (action.type === 'show_notification') {
    let line = `${ind}show_notification(notification_id = "${escapeString(action.notification_id)}", text = "${escapeString(action.text)}"`;
    if (action.duration_ms) {
      line += `, duration_ms = ${action.duration_ms}`;
    }
    lines.push(`${line})`);
  } else {
    lines.push(`${ind}# unknown action: ${JSON.stringify(action)}`);
  }
}

function escapeString(str: string): string {
  return str
    .replace(/\\/g, '\\\\')
    .replace(/"/g, '\\"')
    .replace(/\n/g, '\\n')
    .replace(/\r/g, '\\r');
}

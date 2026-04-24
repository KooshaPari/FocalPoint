import { describe, it, expect } from 'vitest';
import { graphToFpl } from './graphToFpl';
import { SAMPLE_RULE_NODES, SAMPLE_RULE_EDGES } from './sampleRule';

describe('graphToFpl', () => {
  it('should generate FPL from sample rule graph', () => {
    const fpl = graphToFpl(SAMPLE_RULE_NODES, SAMPLE_RULE_EDGES);

    // Should contain the rule name
    expect(fpl).toContain('rule("deep-work-starter")');

    // Should contain the metadata
    expect(fpl).toContain('priority = 10');
    expect(fpl).toContain('cooldown_seconds = 300');
    expect(fpl).toContain('duration_seconds = 3600');
    expect(fpl).toContain('enabled = true');

    // Should contain trigger
    expect(fpl).toContain('trigger');
    expect(fpl).toContain('type = "Event"');

    // Should contain condition
    expect(fpl).toContain('when');
    expect(fpl).toContain('time_in_range');

    // Should contain actions
    expect(fpl).toContain('then');
    expect(fpl).toContain('Block');
    expect(fpl).toContain('Notify');
  });

  it('should handle empty graph', () => {
    const fpl = graphToFpl([], []);
    expect(fpl).toContain('rule("empty")');
  });

  it('should handle missing rule metadata', () => {
    const nodes = [
      {
        id: 'trigger-1',
        type: 'trigger' as const,
        data: { triggerType: 'Event' as const, value: 'test' },
        position: { x: 0, y: 0 },
      },
    ];
    const fpl = graphToFpl(nodes as any, []);
    expect(fpl).toContain('rule("unnamed")');
  });
});

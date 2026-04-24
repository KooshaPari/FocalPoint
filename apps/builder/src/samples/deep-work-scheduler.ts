import { GraphNode, GraphEdge } from '@/types/graph';

export const DEEP_WORK_SCHEDULER = {
  nodes: [
    {
      id: 'rule-meta-deepwork',
      type: 'ruleMeta',
      data: {
        name: 'deep-work-block',
        priority: 10,
        cooldown_seconds: 600,
        duration_seconds: 3600,
        explanation_template: 'Deep work block activated: distractions blocked',
        enabled: true,
      },
      position: { x: 100, y: 50 },
    } as GraphNode,
    {
      id: 'schedule-deepwork',
      type: 'schedule',
      data: {
        cron_spec: '0 9 * * 1-5',
        description: 'Weekday mornings at 9 AM',
        enabled: true,
      },
      position: { x: 100, y: 200 },
    } as GraphNode,
    {
      id: 'condition-deepwork-notweekend',
      type: 'condition',
      data: {
        conditionType: 'day_of_week',
        params: { days: [1, 2, 3, 4, 5] },
        label: 'Weekday Only',
      },
      position: { x: 300, y: 200 },
    } as GraphNode,
    {
      id: 'action-deepwork-block',
      type: 'action',
      data: {
        actionType: 'Block',
        params: { policy: 'social-media-distractions' },
        label: 'Block Distractions',
      },
      position: { x: 500, y: 150 },
    } as GraphNode,
    {
      id: 'action-deepwork-mascot',
      type: 'mascotScene',
      data: {
        pose: 'sitting',
        accessory: 'headphones',
        emotion: 'focused',
        bubble: 'Time to focus!',
        sound_cue: 'focus_mode_activate',
        haptic: 'double_tap',
        entry_anim: 'slide_in',
        hold_ms: 3000,
        exit_anim: 'fade_out',
      },
      position: { x: 500, y: 280 },
    } as GraphNode,
  ] as GraphNode[],
  edges: [
    {
      id: 'edge-deepwork-schedule-condition',
      source: 'schedule-deepwork',
      target: 'condition-deepwork-notweekend',
    } as GraphEdge,
    {
      id: 'edge-deepwork-condition-block',
      source: 'condition-deepwork-notweekend',
      target: 'action-deepwork-block',
    } as GraphEdge,
    {
      id: 'edge-deepwork-block-mascot',
      source: 'action-deepwork-block',
      target: 'action-deepwork-mascot',
    } as GraphEdge,
  ] as GraphEdge[],
};

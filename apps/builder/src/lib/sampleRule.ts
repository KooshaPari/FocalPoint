import { GraphNode, GraphEdge } from '@/types/graph';

export const SAMPLE_RULE_NODES: GraphNode[] = [
  {
    id: 'rule-meta-1',
    type: 'ruleMeta',
    data: {
      name: 'deep-work-starter',
      priority: 10,
      cooldown_seconds: 300,
      duration_seconds: 3600,
      explanation_template: 'Focus session unlocked: blocking social media',
      enabled: true,
    },
    position: { x: 50, y: 50 },
  },
  {
    id: 'trigger-1',
    type: 'trigger',
    data: {
      triggerType: 'Event',
      value: 'focus_session_start',
      label: 'Focus Session Triggered',
    },
    position: { x: 50, y: 200 },
  },
  {
    id: 'condition-1',
    type: 'condition',
    data: {
      conditionType: 'time_in_range',
      params: { start: 8, end: 18 },
      label: 'Daytime Hours',
    },
    position: { x: 250, y: 200 },
  },
  {
    id: 'action-1',
    type: 'action',
    data: {
      actionType: 'Block',
      params: { policy: 'social-media-block' },
      label: 'Block Social Media',
    },
    position: { x: 450, y: 200 },
  },
  {
    id: 'action-2',
    type: 'action',
    data: {
      actionType: 'Notify',
      params: { message: 'Focus mode activated' },
      label: 'Send Notification',
    },
    position: { x: 450, y: 320 },
  },
];

export const SAMPLE_RULE_EDGES: GraphEdge[] = [
  {
    id: 'edge-trigger-condition',
    source: 'trigger-1',
    target: 'condition-1',
    type: 'smoothstep',
  },
  {
    id: 'edge-condition-action1',
    source: 'condition-1',
    target: 'action-1',
    type: 'smoothstep',
  },
  {
    id: 'edge-action1-action2',
    source: 'action-1',
    target: 'action-2',
    type: 'smoothstep',
  },
];

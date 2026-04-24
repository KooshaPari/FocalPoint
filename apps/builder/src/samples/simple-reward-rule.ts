import { GraphNode, GraphEdge } from '@/types/graph';

export const SIMPLE_REWARD_RULE = {
  nodes: [
    {
      id: 'rule-meta-simple',
      type: 'ruleMeta',
      data: {
        name: 'simple-reward',
        priority: 5,
        cooldown_seconds: 60,
        duration_seconds: 0,
        explanation_template: 'You earned credit for completing an action',
        enabled: true,
      },
      position: { x: 100, y: 50 },
    } as GraphNode,
    {
      id: 'trigger-simple',
      type: 'trigger',
      data: {
        triggerType: 'Event',
        value: 'action_completed',
        label: 'Action Completed',
      },
      position: { x: 100, y: 200 },
    } as GraphNode,
    {
      id: 'action-simple-grant',
      type: 'action',
      data: {
        actionType: 'GrantCredit',
        params: { amount: 10, reason: 'action_completed' },
        label: 'Grant 10 Credits',
      },
      position: { x: 350, y: 200 },
    } as GraphNode,
  ] as GraphNode[],
  edges: [
    {
      id: 'edge-simple-trigger-action',
      source: 'trigger-simple',
      target: 'action-simple-grant',
    } as GraphEdge,
  ] as GraphEdge[],
};

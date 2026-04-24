import { GraphNode, GraphEdge } from '@/types/graph';

export const CANVAS_ASSIGNMENT_TRACKER = {
  nodes: [
    {
      id: 'rule-meta-canvas',
      type: 'ruleMeta',
      data: {
        name: 'canvas-assignment-reward',
        priority: 8,
        cooldown_seconds: 300,
        duration_seconds: 0,
        explanation_template: 'Assignment submission detected and rewarded',
        enabled: true,
      },
      position: { x: 100, y: 50 },
    } as GraphNode,
    {
      id: 'trigger-canvas',
      type: 'trigger',
      data: {
        triggerType: 'Event',
        value: 'canvas_submission',
        label: 'Canvas Submission Detected',
      },
      position: { x: 100, y: 200 },
    } as GraphNode,
    {
      id: 'condition-canvas-time',
      type: 'condition',
      data: {
        conditionType: 'time_in_range',
        params: { start: 8, end: 22 },
        label: 'During School Hours',
      },
      position: { x: 300, y: 200 },
    } as GraphNode,
    {
      id: 'action-canvas-grant',
      type: 'action',
      data: {
        actionType: 'GrantCredit',
        params: { amount: 25, reason: 'assignment_completion' },
        label: 'Grant 25 Credits',
      },
      position: { x: 500, y: 150 },
    } as GraphNode,
    {
      id: 'action-canvas-notify',
      type: 'action',
      data: {
        actionType: 'Notify',
        params: { message: 'Great work on your assignment submission!' },
        label: 'Send Notification',
      },
      position: { x: 500, y: 280 },
    } as GraphNode,
  ] as GraphNode[],
  edges: [
    {
      id: 'edge-canvas-trigger-condition',
      source: 'trigger-canvas',
      target: 'condition-canvas-time',
    } as GraphEdge,
    {
      id: 'edge-canvas-condition-grant',
      source: 'condition-canvas-time',
      target: 'action-canvas-grant',
    } as GraphEdge,
    {
      id: 'edge-canvas-grant-notify',
      source: 'action-canvas-grant',
      target: 'action-canvas-notify',
    } as GraphEdge,
  ] as GraphEdge[],
};

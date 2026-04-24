import { GraphNode, GraphEdge } from '@/types/graph';

export const WORKOUT_STREAK_REWARDER = {
  nodes: [
    {
      id: 'rule-meta-workout',
      type: 'ruleMeta',
      data: {
        name: 'workout-streak-bonus',
        priority: 7,
        cooldown_seconds: 120,
        duration_seconds: 86400,
        explanation_template: 'Workout completed! Streak bonus applied',
        enabled: true,
      },
      position: { x: 100, y: 50 },
    } as GraphNode,
    {
      id: 'trigger-workout',
      type: 'trigger',
      data: {
        triggerType: 'Event',
        value: 'workout_completed',
        label: 'Workout Detected',
      },
      position: { x: 100, y: 200 },
    } as GraphNode,
    {
      id: 'condition-workout-streak',
      type: 'condition',
      data: {
        conditionType: 'consecutive_days',
        params: { min_days: 1 },
        label: 'Check Streak',
      },
      position: { x: 300, y: 200 },
    } as GraphNode,
    {
      id: 'action-workout-increment',
      type: 'action',
      data: {
        actionType: 'StreakIncrement',
        params: { metric: 'workouts' },
        label: 'Increment Streak',
      },
      position: { x: 500, y: 120 },
    } as GraphNode,
    {
      id: 'action-workout-grant-base',
      type: 'action',
      data: {
        actionType: 'GrantCredit',
        params: { amount: 50, reason: 'workout_completion' },
        label: 'Grant 50 Credits',
      },
      position: { x: 500, y: 220 },
    } as GraphNode,
    {
      id: 'action-workout-grant-bonus',
      type: 'action',
      data: {
        actionType: 'GrantCredit',
        params: { amount: 100, reason: 'streak_bonus' },
        label: 'Bonus: 100 More',
      },
      position: { x: 500, y: 330 },
    } as GraphNode,
  ] as GraphNode[],
  edges: [
    {
      id: 'edge-workout-trigger-condition',
      source: 'trigger-workout',
      target: 'condition-workout-streak',
    } as GraphEdge,
    {
      id: 'edge-workout-condition-increment',
      source: 'condition-workout-streak',
      target: 'action-workout-increment',
    } as GraphEdge,
    {
      id: 'edge-workout-increment-grant',
      source: 'action-workout-increment',
      target: 'action-workout-grant-base',
    } as GraphEdge,
    {
      id: 'edge-workout-grant-bonus',
      source: 'action-workout-grant-base',
      target: 'action-workout-grant-bonus',
    } as GraphEdge,
  ] as GraphEdge[],
};

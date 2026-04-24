import { GraphNode, GraphEdge } from '@/types/graph';
import { SIMPLE_REWARD_RULE } from './simple-reward-rule';
import { CANVAS_ASSIGNMENT_TRACKER } from './canvas-assignment-tracker';
import { DEEP_WORK_SCHEDULER } from './deep-work-scheduler';
import { WORKOUT_STREAK_REWARDER } from './workout-streak-rewarder';
import { GITHUB_PR_CELEBRATION } from './github-pr-celebration';

export interface SampleTemplate {
  id: string;
  name: string;
  description: string;
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export const SAMPLE_TEMPLATES: SampleTemplate[] = [
  {
    id: 'simple-reward',
    name: 'Simple Reward Rule',
    description: 'Basic rule: trigger event → grant credit',
    nodes: SIMPLE_REWARD_RULE.nodes,
    edges: SIMPLE_REWARD_RULE.edges,
  },
  {
    id: 'canvas-tracker',
    name: 'Canvas Assignment Tracker',
    description: 'Detect assignment submissions and reward completion',
    nodes: CANVAS_ASSIGNMENT_TRACKER.nodes,
    edges: CANVAS_ASSIGNMENT_TRACKER.edges,
  },
  {
    id: 'deep-work',
    name: 'Deep Work Scheduler',
    description: 'Schedule deep work blocks and enforce focus mode',
    nodes: DEEP_WORK_SCHEDULER.nodes,
    edges: DEEP_WORK_SCHEDULER.edges,
  },
  {
    id: 'workout-streak',
    name: 'Workout Streak Rewarder',
    description: 'Reward consecutive workouts with streak bonuses',
    nodes: WORKOUT_STREAK_REWARDER.nodes,
    edges: WORKOUT_STREAK_REWARDER.edges,
  },
  {
    id: 'github-pr',
    name: 'GitHub PR Celebration',
    description: 'Detect merged PRs and celebrate with mascot animation',
    nodes: GITHUB_PR_CELEBRATION.nodes,
    edges: GITHUB_PR_CELEBRATION.edges,
  },
];

import { GraphNode, GraphEdge } from '@/types/graph';

export const GITHUB_PR_CELEBRATION = {
  nodes: [
    {
      id: 'rule-meta-github',
      type: 'ruleMeta',
      data: {
        name: 'github-pr-celebration',
        priority: 6,
        cooldown_seconds: 30,
        duration_seconds: 0,
        explanation_template: 'PR merged! Celebration triggered',
        enabled: true,
      },
      position: { x: 100, y: 50 },
    } as GraphNode,
    {
      id: 'connector-github',
      type: 'connector',
      data: {
        id: 'github-webhook',
        tier: 'free',
        auth: 'oauth2',
        cadence_seconds: 60,
        scopes: ['repo:status', 'repo:public_repo'],
        event_types: ['pull_request.closed'],
      },
      position: { x: 100, y: 200 },
    } as GraphNode,
    {
      id: 'trigger-pr-merge',
      type: 'trigger',
      data: {
        triggerType: 'Event',
        value: 'github_pr_merged',
        label: 'PR Merged',
      },
      position: { x: 300, y: 200 },
    } as GraphNode,
    {
      id: 'action-grant-pr',
      type: 'action',
      data: {
        actionType: 'GrantCredit',
        params: { amount: 75, reason: 'pr_merge' },
        label: 'Grant 75 Credits',
      },
      position: { x: 500, y: 150 },
    } as GraphNode,
    {
      id: 'action-mascot-celebration',
      type: 'mascotScene',
      data: {
        pose: 'jumping',
        accessory: 'party_hat',
        emotion: 'excited',
        bubble: 'Amazing PR merged! 🎉',
        sound_cue: 'celebration_fanfare',
        haptic: 'success_pattern',
        entry_anim: 'bounce_in',
        hold_ms: 2500,
        exit_anim: 'fade_out',
      },
      position: { x: 500, y: 280 },
    } as GraphNode,
  ] as GraphNode[],
  edges: [
    {
      id: 'edge-github-connector-trigger',
      source: 'connector-github',
      target: 'trigger-pr-merge',
    } as GraphEdge,
    {
      id: 'edge-github-trigger-grant',
      source: 'trigger-pr-merge',
      target: 'action-grant-pr',
    } as GraphEdge,
    {
      id: 'edge-github-grant-mascot',
      source: 'action-grant-pr',
      target: 'action-mascot-celebration',
    } as GraphEdge,
  ] as GraphEdge[],
};

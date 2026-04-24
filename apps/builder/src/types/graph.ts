import { Node, Edge } from '@xyflow/react';

export type TriggerType = 'Event' | 'Schedule' | 'StateChange';
export type ConditionType =
  | 'time_in_range'
  | 'day_of_week'
  | 'user_attribute'
  | 'event_property'
  | 'and'
  | 'or'
  | 'not'
  | 'streak_above'
  | 'credit_below'
  | 'policy_active'
  | 'consecutive_days'
  | 'weekend_hour';
export type ActionType =
  | 'GrantCredit'
  | 'DeductCredit'
  | 'Block'
  | 'Unblock'
  | 'StreakIncrement'
  | 'StreakReset'
  | 'Notify';

export interface TriggerNodeData {
  triggerType: TriggerType;
  value: string;
  label?: string;
}

export interface ConditionNodeData {
  conditionType: ConditionType;
  params: Record<string, unknown>;
  label?: string;
}

export interface ActionNodeData {
  actionType: ActionType;
  params: Record<string, unknown>;
  label?: string;
}

export interface RuleMetaNodeData {
  name: string;
  priority: number;
  cooldown_seconds: number;
  duration_seconds: number;
  explanation_template: string;
  enabled: boolean;
}

export interface TaskNodeData {
  title: string;
  duration_minutes: number;
  priority_weight: number;
  deadline: string;
  rigidity: 'soft' | 'hard';
}

export interface ScheduleNodeData {
  cron_spec: string;
  description: string;
  enabled: boolean;
}

export interface ConnectorNodeData {
  id: string;
  tier: 'free' | 'pro' | 'enterprise';
  auth: 'oauth2' | 'apikey' | 'bearer';
  cadence_seconds: number;
  scopes: string[];
  event_types: string[];
}

export interface MascotSceneNodeData {
  pose: string;
  accessory: string;
  emotion: string;
  bubble: string;
  sound_cue: string;
  haptic: string;
  entry_anim: string;
  hold_ms: number;
  exit_anim: string;
}

export interface CoachingConfigNodeData {
  endpoint: string;
  model: string;
  rate_limit_per_min: number;
}

export interface EnforcementPolicyNodeData {
  profile: string;
  targets: string[];
  rigidity: 'soft' | 'hard';
}

export interface WalletMutationNodeData {
  kind: 'grant' | 'spend' | 'streak_inc' | 'streak_reset';
  amount: number;
  purpose: string;
}

export interface RitualNodeData {
  variant: 'morning_brief' | 'evening_shutdown';
  schedule_cron: string;
  top_n_priorities: number;
}

export interface SoundCueNodeData {
  name: string;
  source_url: string;
  loop: boolean;
  gain_db: number;
}

export interface AuditQueryNodeData {
  record_type: string;
  since_hours: number;
}

export type GraphNode = Node<
  TriggerNodeData | ConditionNodeData | ActionNodeData | RuleMetaNodeData |
  TaskNodeData | ScheduleNodeData | ConnectorNodeData | MascotSceneNodeData |
  CoachingConfigNodeData | EnforcementPolicyNodeData | WalletMutationNodeData |
  RitualNodeData | SoundCueNodeData | AuditQueryNodeData,
  'trigger' | 'condition' | 'action' | 'ruleMeta' |
  'task' | 'schedule' | 'connector' | 'mascotScene' |
  'coachingConfig' | 'enforcementPolicy' | 'walletMutation' |
  'ritual' | 'soundCue' | 'auditQuery'
>;
export type GraphEdge = Edge;

export interface GraphState {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

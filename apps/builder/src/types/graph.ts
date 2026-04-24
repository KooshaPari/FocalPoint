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

export type GraphNode = Node<
  TriggerNodeData | ConditionNodeData | ActionNodeData | RuleMetaNodeData,
  'trigger' | 'condition' | 'action' | 'ruleMeta'
>;
export type GraphEdge = Edge;

export interface GraphState {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

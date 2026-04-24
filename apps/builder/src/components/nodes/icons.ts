import {
  Zap, Clock, Settings, Grid, CheckCircle, GitBranch,
  Box, MessageCircle, Settings2, Shield, Wallet, Moon,
  Volume2, Database, MessageSquare, Cog
} from 'lucide-react';

export const NODE_ICONS = {
  // Rule Primitives
  trigger: Zap,
  condition: GitBranch,
  action: CheckCircle,
  ruleMeta: Grid,

  // New 8 Primitives
  task: CheckCircle,
  schedule: Clock,
  connector: Box,
  mascotScene: MessageCircle,
  coachingConfig: Settings2,
  enforcementPolicy: Shield,
  walletMutation: Wallet,
  ritual: Moon,
  soundCue: Volume2,
  auditQuery: Database,
} as const;

export type NodeType = keyof typeof NODE_ICONS;

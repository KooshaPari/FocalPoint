import React from 'react';
import { GraphNode } from '@/types/graph';
import { generateId } from '@/lib/utils';

const PALETTE_ITEMS = [
  {
    category: 'Triggers',
    items: [
      { label: 'Event', type: 'trigger', template: { triggerType: 'Event', value: '' } },
      { label: 'Schedule', type: 'trigger', template: { triggerType: 'Schedule', value: '' } },
      { label: 'State Change', type: 'trigger', template: { triggerType: 'StateChange', value: '' } },
    ],
  },
  {
    category: 'Conditions',
    items: [
      { label: 'Time Range', type: 'condition', template: { conditionType: 'time_in_range', params: { start: 0, end: 24 } } },
      { label: 'Day of Week', type: 'condition', template: { conditionType: 'day_of_week', params: { days: [] } } },
      { label: 'User Attribute', type: 'condition', template: { conditionType: 'user_attribute', params: { key: '', value: '' } } },
      { label: 'Streak Above', type: 'condition', template: { conditionType: 'streak_above', params: { threshold: 0 } } },
      { label: 'Credit Below', type: 'condition', template: { conditionType: 'credit_below', params: { threshold: 0 } } },
    ],
  },
  {
    category: 'Actions',
    items: [
      { label: 'Grant Credit', type: 'action', template: { actionType: 'GrantCredit', params: { amount: 0 } } },
      { label: 'Deduct Credit', type: 'action', template: { actionType: 'DeductCredit', params: { amount: 0 } } },
      { label: 'Block', type: 'action', template: { actionType: 'Block', params: { policy: '' } } },
      { label: 'Unblock', type: 'action', template: { actionType: 'Unblock', params: { policy: '' } } },
      { label: 'Streak Increment', type: 'action', template: { actionType: 'StreakIncrement', params: { key: '' } } },
      { label: 'Streak Reset', type: 'action', template: { actionType: 'StreakReset', params: { key: '' } } },
      { label: 'Notify', type: 'action', template: { actionType: 'Notify', params: { message: '' } } },
    ],
  },
  {
    category: 'Scenes',
    items: [
      { label: 'Mascot Scene', type: 'mascotScene', template: { pose: 'standing', accessory: 'none', emotion: 'happy', bubble: 'Great job!', sound_cue: 'chime', haptic: 'light', entry_anim: 'fade', hold_ms: 2000, exit_anim: 'fade' } },
      { label: 'Ritual', type: 'ritual', template: { variant: 'morning_brief', schedule_cron: '0 8 * * *', top_n_priorities: 3 } },
    ],
  },
  {
    category: 'Config',
    items: [
      { label: 'Rule Meta', type: 'ruleMeta', template: { name: 'Untitled Rule', priority: 1, cooldown_seconds: 0, duration_seconds: 3600, explanation_template: '', enabled: true } },
      { label: 'Coaching Config', type: 'coachingConfig', template: { endpoint: 'https://api.example.com', model: 'gpt-4', rate_limit_per_min: 60 } },
      { label: 'Enforcement Policy', type: 'enforcementPolicy', template: { profile: 'default', targets: [], rigidity: 'soft' } },
      { label: 'Sound Cue', type: 'soundCue', template: { name: 'chime', source_url: '/sounds/chime.mp3', loop: false, gain_db: 0 } },
      { label: 'Connector', type: 'connector', template: { id: 'connector-1', tier: 'free', auth: 'oauth2', cadence_seconds: 300, scopes: [], event_types: [] } },
    ],
  },
  {
    category: 'Data',
    items: [
      { label: 'Task', type: 'task', template: { title: 'New Task', duration_minutes: 30, priority_weight: 1, deadline: '2026-04-24', rigidity: 'soft' } },
      { label: 'Schedule', type: 'schedule', template: { cron_spec: '0 9 * * *', description: 'Daily at 9 AM', enabled: true } },
      { label: 'Wallet Mutation', type: 'walletMutation', template: { kind: 'grant', amount: 10, purpose: 'Daily bonus' } },
      { label: 'Audit Query', type: 'auditQuery', template: { record_type: 'reward', since_hours: 24 } },
    ],
  },
];

interface NodePaletteProps {
  onNodeAdd: (node: GraphNode) => void;
}

export function NodePalette({ onNodeAdd }: NodePaletteProps) {
  const [expanded, setExpanded] = React.useState<Record<string, boolean>>({
    Triggers: true,
    Conditions: false,
    Actions: false,
    Scenes: false,
    Config: false,
    Data: false,
  });

  const toggleCategory = (category: string) => {
    setExpanded(e => ({ ...e, [category]: !e[category] }));
  };

  const addNode = (item: any) => {
    const newNode: GraphNode = {
      id: generateId(item.type),
      type: item.type as any,
      data: item.template,
      position: { x: Math.random() * 200, y: Math.random() * 200 },
    };
    onNodeAdd(newNode);
  };

  return (
    <div className="w-72 bg-white border-r border-gray-200 p-4 overflow-y-auto">
      <h3 className="font-bold text-sm mb-4 text-gray-900">Insert Node</h3>
      {PALETTE_ITEMS.map(category => (
        <div key={category.category} className="mb-3">
          <button
            onClick={() => toggleCategory(category.category)}
            className="w-full flex items-center justify-between px-3 py-2 bg-gray-100 hover:bg-gray-200 rounded font-semibold text-xs text-gray-800 transition"
          >
            <span>{category.category}</span>
            <span className="text-lg">{expanded[category.category] ? '−' : '+'}</span>
          </button>
          {expanded[category.category] && (
            <div className="mt-2 space-y-1">
              {category.items.map(item => (
                <button
                  key={item.label}
                  onClick={() => addNode(item)}
                  className="w-full text-left px-3 py-1.5 text-xs bg-gray-50 hover:bg-gray-100 rounded border border-gray-200 transition"
                >
                  {item.label}
                </button>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

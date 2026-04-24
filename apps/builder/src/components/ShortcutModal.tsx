import React from 'react';

interface ShortcutModalProps {
  open: boolean;
  onClose: () => void;
}

const shortcuts = [
  { key: '?', description: 'Show this help modal' },
  { key: '⌘S', description: 'Save graph to localStorage' },
  { key: '⌘Z', description: 'Undo (via ReactFlow history)' },
  { key: '⌘/', description: 'Validate graph' },
];

export function ShortcutModal({ open, onClose }: ShortcutModalProps) {
  React.useEffect(() => {
    if (!open) return;

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [open]);

  if (!open) return null;

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black bg-opacity-50 z-40"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-lg max-w-md w-full p-6 space-y-4">
          <h2 className="text-xl font-bold text-gray-900">Keyboard Shortcuts</h2>

          <div className="space-y-3">
            {shortcuts.map((shortcut, idx) => (
              <div key={idx} className="flex items-center justify-between text-sm">
                <span className="text-gray-700">{shortcut.description}</span>
                <kbd className="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs font-mono text-gray-900">
                  {shortcut.key}
                </kbd>
              </div>
            ))}
          </div>

          <button
            onClick={onClose}
            className="w-full mt-6 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 transition text-sm font-medium"
          >
            Close (Esc)
          </button>
        </div>
      </div>
    </>
  );
}

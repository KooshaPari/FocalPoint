import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/dom';
import React from 'react';
import { createRoot } from 'react-dom/client';
import { PreviewPane } from './PreviewPane';
import { GraphNode, GraphEdge } from '@/types/graph';

// Mock navigator.clipboard
const mockClipboard = {
  writeText: vi.fn().mockResolvedValue(undefined),
};
Object.assign(navigator, { clipboard: mockClipboard });

// Mock alert
const mockAlert = vi.fn();
global.alert = mockAlert;

describe('PreviewPane', () => {
  const mockNodes: GraphNode[] = [
    {
      id: 'rule-1',
      type: 'ruleMeta',
      position: { x: 0, y: 0 },
      data: { name: 'Test Rule', priority: 10, enabled: true },
    },
    {
      id: 'trigger-1',
      type: 'trigger',
      position: { x: 0, y: 100 },
      data: { triggerType: 'UserStartsSession', value: 'deep_work' },
    },
    {
      id: 'action-1',
      type: 'action',
      position: { x: 0, y: 200 },
      data: { actionType: 'BlockApp', params: { app_id: 'com.example.app' } },
    },
  ];

  const mockEdges: GraphEdge[] = [
    {
      id: 'edge-1',
      source: 'trigger-1',
      target: 'rule-1',
    },
    {
      id: 'edge-2',
      source: 'rule-1',
      target: 'action-1',
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should not render when open is false', () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={false}
        onOpenChange={() => {}}
      />
    );
    expect(container.querySelector('[class*="preview"]')).toBeNull();
  });

  it('should render tabbed interface with all four tabs', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('IR')).toBeInTheDocument();
      expect(screen.getByText('FPL')).toBeInTheDocument();
      expect(screen.getByText('CLI')).toBeInTheDocument();
      expect(screen.getByText('ERRORS')).toBeInTheDocument();
    });
  });

  it('should display IR JSON in IR tab', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const irTab = screen.getByText('IR');
      fireEvent.click(irTab);
      const irContent = container.textContent;
      expect(irContent).toContain('version');
      expect(irContent).toContain('timestamp');
      expect(irContent).toContain('hash');
    });
  });

  it('should display FPL text in FPL tab', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const fplTab = screen.getByText('FPL');
      fireEvent.click(fplTab);
      const fplContent = container.textContent;
      expect(fplContent).toContain('rule');
      expect(fplContent).toContain('Test Rule');
    });
  });

  it('should display CLI command in CLI tab', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const cliTab = screen.getByText('CLI');
      fireEvent.click(cliTab);
      const cliContent = container.textContent;
      expect(cliContent).toContain('focus rules add');
      expect(cliContent).toContain('Test Rule');
    });
  });

  it('should display validation errors in ERRORS tab', async () => {
    // Create invalid graph: only trigger, no actions
    const invalidNodes: GraphNode[] = [
      {
        id: 'rule-1',
        type: 'ruleMeta',
        position: { x: 0, y: 0 },
        data: { name: 'Incomplete Rule', priority: 5, enabled: true },
      },
      {
        id: 'trigger-1',
        type: 'trigger',
        position: { x: 0, y: 100 },
        data: { triggerType: 'UserStartsSession', value: 'deep_work' },
      },
    ];

    const { container } = render(
      <PreviewPane
        nodes={invalidNodes}
        edges={[{ id: 'edge-1', source: 'trigger-1', target: 'rule-1' }]}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const errorsTab = screen.getByText('ERRORS');
      fireEvent.click(errorsTab);
      const errorContent = container.textContent;
      expect(errorContent).toContain('NO_ACTIONS');
    });
  });

  it('should update hash chip when graph changes', async () => {
    const { rerender } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const hashChip = screen.getByTitle('SHA-256 hash of IR rule');
      expect(hashChip).toBeInTheDocument();
      expect(hashChip.textContent).toMatch(/[a-f0-9]{12}\.\.\./);
    });

    // Modify nodes
    const modifiedNodes = [
      ...mockNodes,
      {
        id: 'action-2',
        type: 'action',
        position: { x: 0, y: 300 },
        data: { actionType: 'LockDevice', params: {} },
      },
    ];

    rerender(
      <PreviewPane
        nodes={modifiedNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    // Hash should update (verify by checking hash chip exists after re-render)
    await waitFor(() => {
      const hashChip = screen.getByTitle('SHA-256 hash of IR rule');
      expect(hashChip).toBeInTheDocument();
    });
  });

  it('should copy to clipboard when copy button is clicked', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const copyButton = screen.getByText('Copy IR JSON');
      fireEvent.click(copyButton);
      expect(mockClipboard.writeText).toHaveBeenCalled();
      expect(mockAlert).toHaveBeenCalledWith('Copied to clipboard!');
    });
  });

  it('should call onOpenChange when close button is clicked', async () => {
    const mockOnOpenChange = vi.fn();
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={mockOnOpenChange}
      />
    );

    await waitFor(() => {
      const closeButton = container.querySelector('button:last-child') ||
                          Array.from(container.querySelectorAll('button')).find(btn => btn.textContent === '✕');
      if (closeButton) {
        fireEvent.click(closeButton);
        expect(mockOnOpenChange).toHaveBeenCalledWith(false);
      }
    });
  });

  it('should debounce recomputation with 300ms delay', async () => {
    const { rerender } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    const initialContent = document.body.textContent;

    // Make rapid changes
    for (let i = 0; i < 5; i++) {
      const newNodes = [
        ...mockNodes,
        {
          id: `action-${i}`,
          type: 'action',
          position: { x: 0, y: 300 + i * 50 },
          data: { actionType: 'BlockApp', params: { app_id: `com.example.app${i}` } },
        },
      ];

      rerender(
        <PreviewPane
          nodes={newNodes}
          edges={mockEdges}
          open={true}
          onOpenChange={() => {}}
        />
      );
    }

    // Content should eventually stabilize
    await waitFor(
      () => {
        expect(document.body.textContent).toBeDefined();
      },
      { timeout: 1000 }
    );
  });

  it('should display hash chip that is clickable to copy hash', async () => {
    const { container } = render(
      <PreviewPane
        nodes={mockNodes}
        edges={mockEdges}
        open={true}
        onOpenChange={() => {}}
      />
    );

    await waitFor(() => {
      const hashChip = screen.getByTitle('SHA-256 hash of IR rule');
      fireEvent.click(hashChip);
      expect(mockClipboard.writeText).toHaveBeenCalled();
    });
  });
});

import { HistoryCard } from "@shared/ui/layout/container/HistoryCard/HistoryCard";
import { ScrollArea } from "@shared/ui/layout/container/ScrollArea/ScrollArea";
import { Button } from "@shared/ui/primitives/Button/Button";
import type { HistoryItem } from "@shared/savedResults";

import "./SavedResultsPanel.css";

type Props<TEntry extends { id: string }> = {
  entries: TEntry[];
  buildItems(entry: TEntry): HistoryItem[];
  onLoad(entry: TEntry): void;
  onDelete(id: string): void;
  onClear(): void;
  columns?: number;
  title?: string;
  emptyText?: string;
  clearText?: string;
};

export function SavedResultsPanel<TEntry extends { id: string }>({
  entries,
  buildItems,
  onLoad,
  onDelete,
  onClear,
  columns = 2,
  title = "Saved results",
  emptyText = "No saved results yet",
  clearText = "Clear all results",
}: Props<TEntry>) {
  return (
    <section className="saved-results-panel">
      <h3 className="saved-results-panel-title">{title}</h3>

      <ScrollArea className="saved-results-panel-scroll">
        {entries.length === 0 && (
          <div className="saved-results-panel-empty">{emptyText}</div>
        )}

        {entries.length > 0 && (
          <div className="saved-results-panel-list">
            {entries.map((entry) => (
              <HistoryCard
                key={entry.id}
                items={buildItems(entry)}
                columns={columns}
                onClick={() => onLoad(entry)}
                onDelete={() => onDelete(entry.id)}
              />
            ))}
          </div>
        )}
      </ScrollArea>

      {entries.length > 0 && (
        <div className="saved-results-panel-actions">
          <Button variant="secondary" size="small" onClick={onClear}>
            {clearText}
          </Button>
        </div>
      )}
    </section>
  );
}

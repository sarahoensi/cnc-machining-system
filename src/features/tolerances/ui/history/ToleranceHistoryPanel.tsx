// features/tolerances/ui/history/ToleranceHistoryPanel.tsx

import type { KeyboardEvent, MouseEvent } from "react";

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import type { SavedResultEntry } from "@shared/savedResults";
import { ScrollArea } from "@shared/ui/surfaces/ScrollArea/ScrollArea";
import { Button } from "@shared/ui/primitives/Button/Button";

import type { ToleranceFormState } from "../../domain/toleranceForm";
import { buildToleranceHistoryRow } from "./buildToleranceHistoryRow";
import "./ToleranceHistoryPanel.css";

type Props = {
  history: SavedResultEntry<ToleranceFormState>[];
  onLoad(entry: SavedResultEntry<ToleranceFormState>): void;
  onDelete(id: string): void;
  onClear(): void;
};

export function ToleranceHistoryPanel({
  history,
  onLoad,
  onDelete,
  onClear,
}: Props) {
  const { decimals } = useDisplaySettings();

  function handleRowKeyDown(
    event: KeyboardEvent<HTMLDivElement>,
    entry: SavedResultEntry<ToleranceFormState>,
  ) {
    if (event.key !== "Enter" && event.key !== " ") return;

    event.preventDefault();
    onLoad(entry);
  }

  function handleDelete(
    event: MouseEvent<HTMLButtonElement>,
    id: string,
  ) {
    event.stopPropagation();
    onDelete(id);
  }

  return (
    <section className="tolerance-history">
      <h3 className="tolerance-history-title">Saved results</h3>

      <ScrollArea className="tolerance-history-scroll">
        {history.length === 0 && (
          <div className="tolerance-history-empty">
            No saved results yet
          </div>
        )}

        {history.length > 0 && (
          <div className="tolerance-history-table" role="table">
            <div
              className="tolerance-history-header tolerance-history-grid"
              role="row"
            >
              <span role="columnheader">Type</span>
              <span role="columnheader">Class</span>
              <span className="tolerance-history-nominal" role="columnheader">
                Nominal
              </span>
            </div>

            <div className="tolerance-history-body" role="rowgroup">
              {history.map((entry) => {
                const row = buildToleranceHistoryRow(entry, decimals);

                return (
                  <div
                    key={entry.id}
                    className={`tolerance-history-row tolerance-history-grid ${row.modeClassName}`}
                    role="row"
                    tabIndex={0}
                    onClick={() => onLoad(entry)}
                    onKeyDown={(event) => handleRowKeyDown(event, entry)}
                  >
                    <span className="tolerance-history-type" role="cell">
                      <span className="tolerance-history-type-symbol">
                        {row.symbol}
                      </span>
                      {row.modeLabel}
                    </span>

                    <span className="tolerance-history-class" role="cell">
                      {row.toleranceClass}
                    </span>

                    <span className="tolerance-history-nominal" role="cell">
                      {row.nominal}
                    </span>

                    <span className="tolerance-history-deviations" role="cell">
                      {row.deviations}
                    </span>

                    <span className="tolerance-history-delete-cell" role="cell">
                      <button
                        className="tolerance-history-delete"
                        type="button"
                        aria-label={`Delete saved ${row.modeLabel} result ${row.toleranceClass}`}
                        onClick={(event) => handleDelete(event, entry.id)}
                      >
                        &times;
                      </button>
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
        )}
      </ScrollArea>

      {history.length > 0 && (
        <div className="tolerance-history-actions">
          <Button variant="secondary" size="small" onClick={onClear}>
            Clear all results
          </Button>
        </div>
      )}
    </section>
  );
}


import "./HistoryCard.css";
import type { CSSProperties } from "react";

type Item = {
  label: string;
  value: string;
  unit?: string;
};

type Props = {
  items: Item[];
  columns?: number;
  onClick?: () => void;
  onDelete?: () => void;
};

type HistoryCardStyle = CSSProperties & {
  "--history-card-columns": number;
};

export function HistoryCard({
  items,
  columns = 2,
  onClick,
  onDelete,
}: Props) {
  const contentStyle: HistoryCardStyle = {
    "--history-card-columns": columns,
  };

  return (
    <div className="history-card" onClick={onClick}>
      <div
        className="history-card-content"
        style={contentStyle}
      >
        {items.map((item, i) => (
          <div key={i} className="history-card-item">
            <span className="hc-label">{item.label}</span>

            <span className="hc-value">
              {item.value}
              {item.unit && <span className="hc-unit"> {item.unit}</span>}
            </span>
          </div>
        ))}
      </div>

      {onDelete && (
        <button
          className="history-card-delete"
          onClick={(event) => {
            event.stopPropagation();
            onDelete();
          }}
        >
          &times;
        </button>
      )}
    </div>
  );
}

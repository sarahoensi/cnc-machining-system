import "./HistoryCard.css";

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

export function HistoryCard({
  items,
  columns = 2,
  onClick,
  onDelete,
}: Props) {
  return (
    <div className="history-card" onClick={onClick}>
      <div
        className="history-card-content"
        style={{
          ["--history-card-columns" as any]: columns,
        }}
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

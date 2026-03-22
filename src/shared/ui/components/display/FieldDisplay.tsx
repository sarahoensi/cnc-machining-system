// shared/ui/components/display/FieldDisplay.tsx
import "./FieldDisplay.css";

type Props = {
  label: string;
  value: string;
  unit?: string;
  highlight?: boolean;
  align?: "left" | "right";
};

export function FieldDisplay({
  label,
  value,
  unit,
  highlight = false,
  align = "right",
}: Props) {
  return (
    <div
      className={[
        "field-display",
        highlight ? "highlight" : "",
        align === "left" ? "align-left" : "",
      ].join(" ")}
    >
      <span className="field-display-label">
        {label}
      </span>

      <span className="field-display-value">
        {value}
        {unit && (
          <span className="unit"> {unit}</span>
        )}
      </span>
    </div>
  );
}
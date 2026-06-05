// shared/ui/components/display/FieldDisplay.tsx
import clsx from "clsx";
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
      className={clsx(
        "field-display",
        highlight && "highlight",
        align === "left" && "align-left"
      )}
    >
      <span className="field-display-label">
        {label}
      </span>

      <span className="field-display-value">
        {value}

        {unit && (
          <span className="field-display-unit">
            {unit}
          </span>
        )}
      </span>
    </div>
  );
}
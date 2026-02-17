import { ReactNode } from "react";
import clsx from "clsx";
import { LabelWithTooltip } from "../../form/LabelWithToolTip/LabelWithTooltip";
import "./TableHeaderCell.css";

type Props = {
  label?: string;
  tooltip?: string;
  align?: "left" | "center" | "right";
  children?: ReactNode;
};

export function TableHeaderCell({
  label,
  tooltip,
  align = "left",
  children,
}: Props) {
  return (
    <th
      className={clsx(
        "table-header-cell",
        `align-${align}`
      )}
    >
      <div className="header-content">
        {label && (
          <LabelWithTooltip
            label={label}
            tooltip={tooltip}
          />
        )}

        {children}
      </div>
    </th>
  );
}

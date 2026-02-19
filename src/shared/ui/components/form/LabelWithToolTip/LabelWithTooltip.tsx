// shared/ui/components/form/LabelWithToolTip/LabelWithTooltip.tsx

import TooltipIcon from "../../../../../assets/tooltip-icon.svg";
import clsx from "clsx";
import "./LabelWithTooltip.css";

type Props = {
  label: string;
  tooltip?: string;
  className?: string;
};

export function LabelWithTooltip({
  label,
  tooltip,
  className,
}: Props) {
  return (
    <span className={clsx("label-with-tooltip", className)}>
      <span className="label-text">{label}</span>

      {tooltip && (
        <span
          className="nf-tooltip-icon"
          aria-hidden="true"
          title={tooltip}
        >
          <img src={TooltipIcon} alt="" />
        </span>
      )}
    </span>
  );
}

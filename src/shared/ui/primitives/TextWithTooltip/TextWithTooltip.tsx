// shared/ui/components/primitives/TextWithTooltip/TextWithTooltip.tsx

import clsx from "clsx";
import TooltipIcon from "../../../../assets/tooltip-icon.svg";
import "./TextWithTooltip.css";

type Props = {
  text: string;
  tooltip?: string;
  className?: string;
};

export function TextWithTooltip({
  text,
  tooltip,
  className,
}: Props) {
  return (
    <span className={clsx("text-with-tooltip", className)}>
      <span className="twt-text">{text}</span>

      {tooltip && (
        <span
          className="tooltip-icon"
          aria-hidden="true"
          title={tooltip}
        >
          <img src={TooltipIcon} alt="" />
        </span>
      )}
    </span>
  );
}
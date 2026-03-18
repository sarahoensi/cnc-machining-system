// shared/ui/components/Field/Field.tsx

/**
 * Field - Rammen rundt en input
 * - label
 * - control
 * - error
 * 
 * Vet hvordan label vises, hvor input plasseres, hvor error vises
 */

import { ReactNode } from "react";
import clsx from "clsx";
import { TextWithTooltip } from "../../../primitives/TextWithTooltip/TextWithTooltip";
import "./Field.css";

type Props = {
  label: string;
  tooltip?: string;
  error?: string;
  htmlFor?: string;
  children: ReactNode;
  className?: string;
};

export function Field({
  label,
  tooltip,
  error,
  htmlFor,
  children,
  className,
}: Props) {
  return (
    <div
      className={clsx(
        "field",
        className,
        error && "has-error"
      )}
    >
      <label className="field-label" htmlFor={htmlFor}>
        <TextWithTooltip
          text={label}
          tooltip={tooltip}
        />
      </label>

      <div className="field-control">
        {children}
      </div>

      {error && (
        <div className="field-error">
          {error}
        </div>
      )}
    </div>
  );
}
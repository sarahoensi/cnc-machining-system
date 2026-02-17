// shares/ui/components/Field/Fields.tsx

import { ReactNode } from "react";
import { LabelWithTooltip } from "../LabelWithToolTip/LabelWithTooltip";
import clsx from "clsx";
import "./Field.css";

type Props = {
  label: string;
  tooltip?: string;
  error?: string;
  htmlFor?: string;
  as?: "div" | "fieldset";
  children: ReactNode;
};

export function Field({
  label,
  tooltip,
  error,
  htmlFor,
  as = "div",
  children,
}: Props) {
  const Component = as;

  if (as === "fieldset") {
    return (
      <fieldset className={clsx("field", error && "has-error")}>
        <legend className="field-label">
          <LabelWithTooltip label={label} tooltip={tooltip} />
        </legend>

        <div className="field-control">{children}</div>

        {error && <div className="field-error">{error}</div>}
      </fieldset>
    );
  }

  return (
    <Component className={clsx("field", error && "has-error")}>
      <label className="field-label" htmlFor={htmlFor}>
        <LabelWithTooltip label={label} tooltip={tooltip} />
      </label>

      <div className="field-control">{children}</div>

      {error && <div className="field-error">{error}</div>}
    </Component>
  );
}

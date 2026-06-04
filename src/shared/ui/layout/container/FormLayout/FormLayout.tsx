// shared/ui/layout/container/FormLayout/FormLayout.tsx

import clsx from "clsx";
import type { ReactNode, Ref } from "react";
import "./FormLayout.css";

export function FormLayout({
  fields,
  error,
  actions,
  actionsPlacement = "flow",
  containerRef,
}: {
  fields: ReactNode;
  error?: ReactNode;
  actions: ReactNode;
  actionsPlacement?: "flow" | "bottom";
  containerRef?: Ref<HTMLDivElement>;
}) {
  return (
    <div
      ref={containerRef}
      className={clsx(
        "form-layout",
        actionsPlacement === "bottom" && "form-layout--actions-bottom",
      )}
    >
      <div className="form-fields">{fields}</div>

      {error && (
        <div className="form-error-block">
          {error}
        </div>
      )}

      <div className="form-layout-actions-slot">
        {actions}
      </div>
    </div>
  );
}

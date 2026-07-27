// shared/ui/components/form/FormLayout/FormLayout.tsx

import clsx from "clsx";
import type { ReactNode } from "react";
import "./FormLayout.css";

export function FormLayout({
  children,
  error,
  actions,
  actionsPlacement = "flow",
  className,
}: {
  children: ReactNode;
  error?: ReactNode;
  actions?: ReactNode;
  actionsPlacement?: "flow" | "bottom";
  className?: string;
}) {
  return (
    <div
      className={clsx(
        "form-layout",
        actionsPlacement === "bottom" && "form-layout--actions-bottom",
        className,
      )}
    >
      <div className="form-fields">{children}</div>

      {error && <div className="form-error-block">{error}</div>}

      {actions && <div className="form-layout-actions-slot">{actions}</div>}
    </div>
  );
}

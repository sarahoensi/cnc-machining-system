// src/shared/ui/layout/container/Panel/Panel.tsx

//

import clsx from "clsx";
import "./Panel.css";

type Props = {
  title?: React.ReactNode;
  children: React.ReactNode;
  actions?: React.ReactNode;
  className?: string;
};

export function Panel({
  title,
  children,
  actions,
  className,
}: Props) {
  return (
    <div className={clsx("panel", className)}>

      {title && (
        <div className="panel-title">
          {title}
        </div>
      )}

      <div className="panel-content">
        {children}
      </div>

      {actions && (
        <div className="panel-actions">
          {actions}
        </div>
      )}

    </div>
  );
}

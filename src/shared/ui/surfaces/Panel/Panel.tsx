import clsx from "clsx";
import type { ReactNode } from "react";

import "./Panel.css";

type Props = {
  title?: ReactNode;
  children: ReactNode;
  actions?: ReactNode;
  className?: string;
};

export function Panel({ title, children, actions, className }: Props) {
  return (
    <div className={clsx("panel", className)}>
      {title && <div className="panel-title">{title}</div>}
      <div className="panel-content">{children}</div>
      {actions && <div className="panel-actions">{actions}</div>}
    </div>
  );
}

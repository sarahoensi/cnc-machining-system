import clsx from "clsx";
import type { ReactNode } from "react";

import "./PageShell.css";

type Props = {
  children: ReactNode;
  className?: string;
};

export function PageShell({ children, className }: Props) {
  return <div className={clsx("page-shell", className)}>{children}</div>;
}

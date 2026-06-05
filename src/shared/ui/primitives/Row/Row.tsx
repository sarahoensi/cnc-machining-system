import clsx from "clsx";
import type { ReactNode } from "react";

import "./Row.css";

type Props = {
  children: ReactNode;
  columns?: 1 | 2 | 3;
  className?: string;
};

export function Row({
  children,
  columns = 1,
  className,
}: Props) {
  return (
    <div className={clsx("row", `row--columns-${columns}`, className)}>
      {children}
    </div>
  );
}

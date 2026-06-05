import clsx from "clsx";
import type { ReactNode } from "react";

import "./ScrollArea.css";

type Props = {
  children: ReactNode;
  className?: string;
};

export function ScrollArea({ children, className }: Props) {
  return <div className={clsx("scroll-area", className)}>{children}</div>;
}

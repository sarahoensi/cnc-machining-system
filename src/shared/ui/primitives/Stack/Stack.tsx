import clsx from "clsx";
import type { ReactNode } from "react";

import "./Stack.css";

type Props = {
  children: ReactNode;
  className?: string;
};

export function Stack({ children, className }: Props) {
  return <div className={clsx("stack", className)}>{children}</div>;
}

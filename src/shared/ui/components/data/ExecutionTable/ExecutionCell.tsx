// shared/ui/components/data/ExecutionTable/ExecutionTable.tsx

import { ReactNode } from "react";
import clsx from "clsx";

type Props = {
  children: ReactNode;
  align?: "left" | "center" | "right";
  className?: string;
};

export function ExecutionCell({
  children,
  align = "left",
  className,
}: Props) {
  return (
    <td
      className={clsx(
        "execution-cell",
        `align-${align}`,
        className
      )}
    >
      {children}
    </td>
  );
}

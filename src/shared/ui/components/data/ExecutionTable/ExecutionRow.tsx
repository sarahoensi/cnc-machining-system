// shared/ui/components/data/ExecutionRow.tsx

import { ReactNode } from "react";
import clsx from "clsx";

type Props = {
  children: ReactNode;
  isActive?: boolean;
  isSelected?: boolean;
};

export function ExecutionRow({
  children,
  isActive,
  isSelected,
}: Props) {
  return (
    <tr
      className={clsx(
        isActive && "active",
        isSelected && "selected"
      )}
    >
      {children}
    </tr>
  );
}

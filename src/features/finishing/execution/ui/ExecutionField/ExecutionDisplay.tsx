// src/shared/ui/components/execution/ExecutionDisplay.tsx

//

import { ReactNode } from "react";
import clsx from "clsx";
import "./ExecutionField.css";

type Props = {
  children: ReactNode;
  className?: string;
};

export function ExecutionDisplay({ children, className }: Props) {
  return (
    <div className={clsx("exec-field", className)}>
      <div className="exec-display">{children}</div>
    </div>
  );
}

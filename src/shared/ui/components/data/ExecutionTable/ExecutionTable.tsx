// shares/ui/components/data/ExecutionTable/ExecutionTable.tsx

import { ReactNode } from "react";
import clsx from "clsx";
import "./ExecutionTable.css";

type Props = {
  headers: ReactNode; // ← endret fra array
  children: ReactNode;
  footer?: ReactNode;
  className?: string;
};

export function ExecutionTable({
  headers,
  children,
  footer,
  className,
}: Props) {
  return (
    <table className={clsx("execution-table", className)}>
      <thead className="execution-table-head">
        <tr>{headers}</tr>
      </thead>

      <tbody className="execution-table-body">
        {children}
      </tbody>

      {footer && (
        <tfoot className="execution-table-foot">
          {footer}
        </tfoot>
      )}
    </table>
  );
}

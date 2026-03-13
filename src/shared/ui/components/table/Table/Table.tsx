// shared/ui/components/data/Table/Table.tsx

import { ReactNode } from "react";
import clsx from "clsx";
import "./Table.css";

type TableProps = {
  children: ReactNode;
  className?: string;
};

function Root({ children, className }: TableProps) {
  return (
    <table className={clsx("ui-table", className)}>
      {children}
    </table>
  );
}

function Head({ children }: { children: ReactNode }) {
  return <thead className="ui-table-head">{children}</thead>;
}

function Body({ children }: { children: ReactNode }) {
  return <tbody className="ui-table-body">{children}</tbody>;
}

function HeadRow(props: RowProps) {
  return <Row {...props} />;
}

function BodyRow(props: RowProps) {
  return <Row {...props} />;
}

function Foot({ children }: { children: ReactNode }) {
  return <tfoot className="ui-table-foot">{children}</tfoot>;
}

type RowProps = {
  children: ReactNode;
  isActive?: boolean;
  isSelected?: boolean;
  className?: string;
};

function Row({
  children,
  isActive,
  isSelected,
  className,
}: RowProps) {
  return (
    <tr
      className={clsx(
        "ui-table-row",
        isActive && "active",
        isSelected && "selected",
        className
      )}
    >
      {children}
    </tr>
  );
}

type CellProps = {
  children: ReactNode;
  align?: "left" | "center" | "right";
  padding?: "default" | "none";
  className?: string;
};

function Cell({
  children,
  align = "left",
  className,
}: CellProps) {
  return (
    <td
      className={clsx(
        "ui-table-cell",
        `align-${align}`,
        className
      )}
    >
      {children}
    </td>
  );
}

type HeaderCellProps = {
  children?: ReactNode;
  align?: "left" | "center" | "right";
  className?: string;
};

function HeaderCell({
  children,
  align = "left",
  className,
}: HeaderCellProps) {
  return (
    <th
      className={clsx(
        "ui-table-header-cell",
        `align-${align}`,
        className
      )}
    >
      <div className="ui-table-header-content">
        {children}
      </div>
    </th>
  );
}

export const Table = {
  Root,
  Head,
  Body,
  Foot,
  Row,
  Cell,
  HeaderCell,
  HeadRow,
  BodyRow,
};
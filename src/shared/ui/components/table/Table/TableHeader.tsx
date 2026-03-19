// shared/ui/components/data/Table/TableHeader.tsx

import { ReactNode } from "react";
import { Table } from "./Table";
import { TextWithTooltip } from "../../../primitives/TextWithTooltip/TextWithTooltip";

type Props = {
  label?: string;
  tooltip?: string;
  align?: "left" | "center" | "right";
  children?: ReactNode;
};

export function TableHeader({
  label,
  tooltip,
  align,
  children,
}: Props) {
  return (
    <Table.HeaderCell align={align}>
      {label && (
        <TextWithTooltip
          text={label}
          tooltip={tooltip}
        />
      )}
      {children}
    </Table.HeaderCell>
  );
}
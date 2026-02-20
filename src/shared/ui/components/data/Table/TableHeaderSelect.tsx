// shared/ui/components/data/Table/TableHeaderSelext.tsx

import { ReactNode } from "react";
import { Table } from "./Table";
import "./TableHeaderSelect.css";

type Option<T extends string> = {
  value: T;
  label: ReactNode;
};

type Props<T extends string> = {
  value: T;
  onChange: (value: T) => void;
  options: readonly Option<T>[];
  align?: "left" | "center" | "right";
};

export function TableHeaderSelect<T extends string>({
  value,
  onChange,
  options,
  align = "center",
}: Props<T>) {
  return (
    <Table.HeaderCell align={align}>
      <div className="ths-wrapper">
        <select
          className="ths-select"
          value={value}
          onChange={(e) =>
            onChange(e.target.value as T)
          }
        >
          {options.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
        <span className="ths-caret" />
      </div>
    </Table.HeaderCell>
  );
}
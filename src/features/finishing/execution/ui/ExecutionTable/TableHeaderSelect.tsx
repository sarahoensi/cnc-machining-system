// src/shared/ui/components/table/Table/TableHeaderSelect.tsx

// TableHeaderSelect.tsx

import { ReactNode, useEffect, useRef, useState } from "react";
import { Table } from "@shared/ui/table/Table/Table";
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
  const [open, setOpen] = useState(false);

  const wrapperRef = useRef<HTMLDivElement>(null);

  const selected = options.find((o) => o.value === value);

  /* =========================================
     Close on click outside
  ========================================= */

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (wrapperRef.current && !wrapperRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <Table.HeaderCell align={align}>
      <div className="ths-wrapper" ref={wrapperRef}>
        <button className="ths-button" onClick={() => setOpen((o) => !o)}>
          {selected?.label}
          <span className="ths-caret" />
        </button>

        {open && (
          <div className="ths-menu">
            {options.map((opt) => (
              <button
                key={opt.value}
                className="ths-option"
                onClick={() => {
                  onChange(opt.value);
                  setOpen(false);
                }}
              >
                {opt.label}
              </button>
            ))}
          </div>
        )}
      </div>
    </Table.HeaderCell>
  );
}

import { ReactNode } from "react";
import clsx from "clsx";
import "./SelectMenu.css";

type SelectOption<T extends string> = {
  value: T;
  label: ReactNode;
  meta?: ReactNode;
};

type SelectMenuProps<T extends string> = {
  valueLabel: ReactNode;
  open: boolean;
  onToggle: () => void;
  options: SelectOption<T>[];
  onSelect: (value: T) => void;
  utilityItems?: { label: ReactNode; onClick: () => void }[];
  className?: string;
};

export function SelectMenu<T extends string>({
  valueLabel,
  open,
  onToggle,
  options,
  onSelect,
  utilityItems = [],
  className,
}: SelectMenuProps<T>) {
  return (
    <div className={clsx("app-select-menu", className)}>
      <button
        type="button"
        className="app-select-trigger"
        onClick={onToggle}
      >
        <span className="app-select-trigger-label">{valueLabel}</span>
        <span className="app-select-trigger-caret" />
      </button>

      {open ? (
        <div className="app-select-dropdown">
          {options.map((option) => (
            <button
              key={option.value}
              type="button"
              className="app-select-option"
              onClick={() => onSelect(option.value)}
            >
              <span className="app-select-option-label">{option.label}</span>
              {option.meta ? (
                <span className="app-select-option-meta">{option.meta}</span>
              ) : null}
            </button>
          ))}

          {utilityItems.length > 0 ? (
            <>
              <div className="app-select-divider" />
              {utilityItems.map((item, idx) => (
                <button
                  key={idx}
                  type="button"
                  className="app-select-utility"
                  onClick={item.onClick}
                >
                  {item.label}
                </button>
              ))}
            </>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

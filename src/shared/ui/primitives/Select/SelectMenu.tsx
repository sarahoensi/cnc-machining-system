// src/shared/ui/primitives/Select/SelectMenu.tsx

import { ReactNode } from "react";
import clsx from "clsx";
import type {
  InputAppearance,
  InputSize,
  InputSource,
} from "@shared/ui/primitives/input/types";
import "@shared/ui/primitives/input/InputControl/InputControl.css";
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
  appearance?: InputAppearance;
  source?: InputSource;
  size?: InputSize;
  disabled?: boolean;
};

export function SelectMenu<T extends string>({
  valueLabel,
  open,
  onToggle,
  options,
  onSelect,
  utilityItems = [],
  className,
  appearance = "form",
  source = "default",
  size = "medium",
  disabled = false,
}: SelectMenuProps<T>) {
  return (
    <div className={clsx("app-select-menu", open && !disabled && "is-open", className)}>
      <button
        type="button"
        className={clsx(
          "app-select-trigger",
          "input-control",
          `input-control--${appearance}`,
          `input-control--${size}`,
          source !== "default" && `input-control--${source}`,
          disabled && "input-control--disabled"
        )}
        onClick={() => {
          if (!disabled) {
            onToggle();
          }
        }}
        disabled={disabled}
      >
        <span className="app-select-trigger-label">{valueLabel}</span>
        <span className="app-select-trigger-caret" />
      </button>

      {open && !disabled ? (
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


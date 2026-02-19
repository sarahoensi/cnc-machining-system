// shared/ui/components/primitives/ToggleGroup/ToggleGroup.tsx

import clsx from "clsx";
import "./ToggleGroup.css";

export type ToggleOption<T extends string = string> = {
  value: T;
  label: string;
};

type Props<T extends string> = {
  value: T;
  onChange: (value: T) => void;
  options: readonly ToggleOption<T>[];
  disabled?: boolean;
};

export function ToggleGroup<T extends string>({
  value,
  onChange,
  options,
  disabled = false,
}: Props<T>) {
  return (
    <div
      className={clsx("toggle-group", disabled && "disabled")}
      role="group"
    >
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          className={clsx(
            "toggle-button",
            value === option.value && "active"
          )}
          onClick={() => onChange(option.value)}
          disabled={disabled}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}

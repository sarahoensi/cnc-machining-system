// shared/ui/primitives/RadioGroup/RadioGroup.tsx

import clsx from "clsx";
import { TextWithTooltip } from "../TextWithTooltip/TextWithTooltip";
import "./RadioGroup.css";

export type RadioOption<T extends string = string> = {
  value: T;
  label: string;
  tooltip?: string;
};

type Props<T extends string> = {
  name: string;
  value: T;
  onChange: (value: T) => void;
  options: readonly RadioOption<T>[];

  disabled?: boolean;
  className?: string;
};

export function RadioGroup<T extends string>({
  name,
  value,
  onChange,
  options,
  disabled = false,
  className,
}: Props<T>) {
  function handleChange(optionValue: T) {
    if (disabled) return;
    onChange(optionValue);
  }

  return (
    <div
      className={clsx("radio-group", className, disabled && "is-disabled")}
      role="radiogroup"
      aria-disabled={disabled}
    >
      {options.map((option) => (
        <label key={option.value} className="radio-option">
          <input
            className="radio-input"
            type="radio"
            name={name}
            value={option.value}
            checked={value === option.value}
            onChange={() => handleChange(option.value)}
            disabled={disabled}
          />

          <TextWithTooltip text={option.label} tooltip={option.tooltip} />
        </label>
      ))}
    </div>
  );
}

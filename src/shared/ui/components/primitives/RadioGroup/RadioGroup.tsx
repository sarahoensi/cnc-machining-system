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
  readonly?: boolean;

  className?: string;
};

export function RadioGroup<T extends string>({
  name,
  value,
  onChange,
  options,
  disabled = false,
  readonly = false,
  className,
}: Props<T>) {

  const isDisabled = disabled;
  const isReadonly = readonly && !isDisabled;

  function handleChange(optionValue: T) {
    if (isDisabled || isReadonly) return;
    onChange(optionValue);
  }

  return (
    <div
      className={clsx(
        "radio-group",
        className,
        isDisabled && "is-disabled",
        isReadonly && "is-readonly"
      )}
      role="radiogroup"
      aria-disabled={isDisabled || isReadonly}
    >
      {options.map((option) => (
        <label
          key={option.value}
          className="radio-option"
        >
          <input
            className="radio-input"
            type="radio"
            name={name}
            value={option.value}
            checked={value === option.value}
            onChange={() => handleChange(option.value)}
            disabled={isDisabled}
            tabIndex={isReadonly ? -1 : undefined}
          />

          <TextWithTooltip
            text={option.label}
            tooltip={option.tooltip}
          />
        </label>
      ))}
    </div>
  );
}
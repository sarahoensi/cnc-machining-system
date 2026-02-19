// shared/ui/components/ModeSelector/ModeSelector.tsx

import clsx from "clsx";
import { Field } from "../Field/Field";
import { LabelWithTooltip } from "../LabelWithToolTip/LabelWithTooltip";
import "./ModeSelector.css";

export type RadioOption<T extends string = string> = {
  value: T;
  label: string;
  tooltip?: string;
};

type Props<T extends string> = {
  name: string;
  label: string;
  tooltip?: string;
  value: T;
  onChange: (value: T) => void;
  options: readonly RadioOption<T>[];

  /** Hard disable (domain) */
  disabled?: boolean;

  /** UI-level readonly */
  readonly?: boolean;

  error?: string;
};

export function ModeSelector<T extends string>({
  name,
  label,
  tooltip,
  value,
  onChange,
  options,
  disabled = false,
  readonly = false,
  error,
}: Props<T>) {

  const isDisabled = disabled;
  const isReadOnly = readonly && !isDisabled;

  function handleChange(optionValue: T) {
    if (isDisabled || isReadOnly) return;
    onChange(optionValue);
  }

  return (
    <Field
      as="fieldset"
      label={label}
      tooltip={tooltip}
      error={error}
    >
      <div
        className={clsx(
          "radio-group",
          isDisabled && "disabled",
          isReadOnly && "readonly"
        )}
        aria-disabled={isDisabled || isReadOnly}
      >
        {options.map((option) => (
          <label
            key={option.value}
            className={clsx(
              "radio-option",
              isDisabled && "disabled",
              isReadOnly && "readonly"
            )}
          >
            <input
              type="radio"
              name={name}
              value={option.value}
              checked={value === option.value}
              onChange={() => handleChange(option.value)}
              disabled={isDisabled}
              tabIndex={isReadOnly ? -1 : undefined}
            />

            <LabelWithTooltip
              label={option.label}
              tooltip={option.tooltip}
            />
          </label>
        ))}
      </div>
    </Field>
  );
}

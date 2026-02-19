import React, { useId } from "react";
import { Field } from "../Field/Field";
import { NumberInput } from "../../primitives/NumberInput/NumberInput";
import type { FieldState } from "@shared/types/fields";

type Props = {
  label: string;
  field: FieldState;
  onChange: (value: string) => void;

  tooltip?: string;
  error?: string;
  unit?: string;

  disabled?: boolean;
  readonly?: boolean;

  autoFocus?: boolean;
  inputRef?: React.Ref<HTMLInputElement>;
  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
};

export function FormNumberField({
  label,
  tooltip,
  error,
  field,
  onChange,
  unit,
  disabled,
  readonly,
  autoFocus,
  inputRef,
  onKeyDown,
  onFocus,
  onBlur,
}: Props) {
  const id = useId();

  return (
    <Field
      label={label}
      tooltip={tooltip}
      error={error}
      htmlFor={id}
    >
      <NumberInput
        id={id}
        field={field}
        onChange={onChange}
        unit={unit}
        disabled={disabled}
        readonly={readonly}
        autoFocus={autoFocus}
        inputRef={inputRef}
        onKeyDown={onKeyDown}
        onFocus={onFocus}
        onBlur={onBlur}
      />
    </Field>
  );
}

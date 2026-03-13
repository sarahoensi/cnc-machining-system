// shared/ui/components/form/formNumberField/FormNumberField.tsx

import React, { useId } from "react";
import { Field } from "../Field/Field";
import { NumberInput } from "../../primitives/NumberInput/NumberInput";
import type { FieldState } from "@shared/form/types/fields";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

import "./FormNumberField.css";

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

  const { decimals } = useDisplaySettings();

  const isDisabled = disabled || field.locked;
  const isReadOnly = readonly && !isDisabled;

  const displayValue =
    field.source === "machine" &&
    typeof field.machineValue === "number"
      ? field.machineValue.toFixed(decimals)
      : field.value ?? "";

  return (
    <Field
      label={label}
      tooltip={tooltip}
      error={error}
      htmlFor={id}
      className={`nf-input source-${field.source}`}
    >
      <NumberInput
        id={id}
        value={displayValue}
        onChange={onChange}
        unit={unit}
        disabled={isDisabled}
        readonly={isReadOnly}
        autoFocus={autoFocus}
        inputRef={inputRef}
        onKeyDown={onKeyDown}
        onFocus={onFocus}
        onBlur={onBlur}
      />
    </Field>
  );
}
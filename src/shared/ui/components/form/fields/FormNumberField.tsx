// shared/ui/components/form/fields/FormNumberField.tsx

import React, { forwardRef, useId } from "react";
import { Field } from "../Field/Field";
import { NumberInput } from "../../primitives/NumberInput/NumberInput";
import type { FieldState } from "@shared/form/types/fields";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

//import "./FormNumberField.css";

type Props = {
  label: string;
  field: FieldState;
  onChange: (value: string) => void;

  tooltip?: string;
  unit?: string;

  disabled?: boolean;
  readonly?: boolean;

  autoFocus?: boolean;

  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
};

export const FormNumberField = forwardRef<HTMLInputElement, Props>(
function FormNumberField(
{
  label,
  tooltip,
  field,  
  unit,
  disabled,
  readonly,
  autoFocus,

  onChange,
  onKeyDown,
  onFocus,
},
ref
) {

  const id = useId();
  const { decimals } = useDisplaySettings();

  const isDisabled = disabled || field.locked;
  const isReadonly = readonly && !isDisabled;

  const displayValue =
  field.source === "machine" &&
  typeof field.machineValue === "number"
    ? field.machineValue.toFixed(decimals)
    : field.value ?? "";

    const tabIndex = field.locked ? -1 : undefined;

  return (
    <Field
      label={label}
      tooltip={tooltip}
      error={field.error}
      htmlFor={id}
    >
      <NumberInput
        variant="form"
        id={id}
        ref={ref}
        value={displayValue}
        onChange={onChange}
        unit={unit}
        disabled={isDisabled}
        readonly={isReadonly}
        autoFocus={autoFocus}
        onKeyDown={onKeyDown}
        onFocus={onFocus}
        tabIndex={tabIndex}
      />
    </Field>
  );
});
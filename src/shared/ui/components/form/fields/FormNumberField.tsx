// shared/ui/components/form/fields/FormNumberField.tsx

import React, { forwardRef, useId } from "react";
import { Field } from "../Field/Field";
import { NumberInput } from "@shared/ui/primitives/input";
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
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
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
  onBlur,
},
ref
) {

  const id = useId();
  const { decimals } = useDisplaySettings();

  const isResultField = field.kind === "result";

  const isDisabled = disabled || field.locked;
  const isReadonly = (readonly || isResultField) && !isDisabled;
  const inputSource = field.source === "empty" ? "default" : field.source;

  const displayValue =
  field.source === "machine" &&
  typeof field.machineValue === "number"
    ? field.machineValue.toFixed(decimals)
    : field.value ?? "";

    const tabIndex = field.locked || isResultField ? -1 : undefined;

  return (
    <Field
      label={label}
      tooltip={tooltip}
      error={field.error}
      htmlFor={id}
    >
      <NumberInput
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
        onBlur={onBlur}
        tabIndex={tabIndex}
        appearance="form"
        source={inputSource}
      />
    </Field>
  );
});

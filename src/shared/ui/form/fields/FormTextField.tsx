// src/shared/ui/components/form/fields/FormTextField.tsx

import React, { forwardRef, useId } from "react";
import { Field } from "../Field";
import { TextInput } from "@shared/ui/primitives/input";
import type { InputSize, InputSource } from "@shared/ui/primitives/input";

type Props = {
  label: string;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  tooltip?: string;
  placeholder?: string;
  disabled?: boolean;
  autoFocus?: boolean;
  size?: InputSize;
  source?: InputSource;
  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
};

export const FormTextField = forwardRef<HTMLInputElement, Props>(function FormTextField(
  {
    label,
    value,
    onChange,
    error,
    tooltip,
    placeholder,
    disabled,
    autoFocus,
    size = "medium",
    source = "user",
    onKeyDown,
    onFocus,
    onBlur,
  },
  ref,
) {
  const id = useId();

  return (
    <Field label={label} tooltip={tooltip} error={error} htmlFor={id}>
      <TextInput
        id={id}
        ref={ref}
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        disabled={disabled}
        autoFocus={autoFocus}
        appearance="form"
        size={size}
        source={source}
        onKeyDown={onKeyDown}
        onFocus={onFocus}
        onBlur={onBlur}
      />
    </Field>
  );
});

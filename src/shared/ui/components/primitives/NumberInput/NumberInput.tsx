// shared/ui/primitives/NumberInput/NumberInput.tsx

import React, { forwardRef, useId } from "react";
import clsx from "clsx";
import "./NumberInput.css";

type Props = {
  id?: string;

  value: string;
  onChange: (value: string) => void;

  unit?: string;

  disabled?: boolean;
  readonly?: boolean;

  autoFocus?: boolean;

  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;

  placeholder?: string;
  className?: string;
};

const INPUT_REGEX = /^-?\d*([.,]\d*)?$/;

export const NumberInput = forwardRef<HTMLInputElement, Props>(
function NumberInput(
{
  id,
  value,
  onChange,
  unit,
  disabled = false,
  readonly = false,
  autoFocus,
  onKeyDown,
  onFocus,
  onBlur,
  placeholder,
  className,
},
ref
) {


  const generatedId = useId();
  const inputId = id ?? generatedId;

  const isDisabled = disabled;
  const isReadOnly = readonly && !isDisabled;

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {

    if (isDisabled || isReadOnly) return;

    const raw = e.target.value;

    if (raw === "") {
      onChange("");
      return;
    }

    if (INPUT_REGEX.test(raw)) {
      onChange(raw);
    }
  }

  function handleBlurInternal(e: React.FocusEvent<HTMLInputElement>) {

  if (isDisabled || isReadOnly) {
    onBlur?.(e);
    return;
  }

  onBlur?.(e);
}

  return (
    <div className={clsx("number-input", className)}>
      <div className="ni-input-wrapper">

        <input
          id={inputId}
          ref={ref}
          type="text"
          inputMode="decimal"
          pattern="-?[0-9]*[.,]?[0-9]*"
          autoFocus={autoFocus}
          value={isDisabled ? "" : value}
          disabled={isDisabled}
          readOnly={isReadOnly}
          tabIndex={isReadOnly ? -1 : undefined}
          placeholder={placeholder}
          onChange={handleChange}
          onFocus={onFocus}
          onBlur={handleBlurInternal}
          onKeyDown={onKeyDown}
          className={clsx(
            "ni-input",
            isDisabled && "disabled",
            isReadOnly && "readonly"
          )}
        />

        {unit && (
          <span className="ni-unit">
            {unit}
          </span>
        )}

      </div>
    </div>
  );
});
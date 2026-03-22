// shared/ui/primitives/NumberInput/NumberInput.tsx

import React, { forwardRef, useId } from "react";
import clsx from "clsx";
import "./NumberInput.base.css";
import "./NumberInput.form.css";
import "./NumberInput.execution.css";



type Props = {
  

  id?: string;

  value: string;
  onChange?: (value: string) => void;

  unit?: string;

  disabled?: boolean;
  readonly?: boolean;
  displayOnly?: boolean;

  autoFocus?: boolean;
  tabIndex?: number;

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
  displayOnly = false,
  autoFocus,
  tabIndex,
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

  const isDisplayOnly = displayOnly;
  const isDisabled = disabled;
  const isReadOnly = readonly && !isDisabled;

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {


    const raw = e.target.value;

    if (raw === "") {
      onChange?.("");
      return;
    }

    if (INPUT_REGEX.test(raw)) {
      onChange?.(raw);
    }
  }


  return (
    <div
      className={clsx(
        "number-input",
        isDisabled && "is-disabled",
        isReadOnly && "readonly",
        isDisplayOnly && "is-display-only",
        className
      )}
    >
      <div className="ni-input-wrapper">

        <input
          id={inputId}
          ref={ref}
          type="text"
          inputMode="decimal"
          pattern="-?[0-9]*[.,]?[0-9]*"
          autoFocus={autoFocus}
          value={value}
          disabled={isDisabled}
          readOnly={isReadOnly}
          tabIndex={tabIndex}
          placeholder={placeholder}
          onChange={handleChange}
          onFocus={onFocus}
          onBlur={onBlur}
          onKeyDown={onKeyDown}
          className="ni-input"
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
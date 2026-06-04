// shared/ui/primitives/NumberInput/NumberInput.tsx

import React, { forwardRef, useId } from "react";
import clsx from "clsx";
import { InputBase } from "@shared/ui/primitives/input/InputBase";
import type {
  InputAppearance,
  InputSize,
  InputSource,
} from "@shared/ui/primitives/input/types";
import "@shared/ui/primitives/input/InputControl/InputControl.css";
import "./NumberInput.base.css";



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
  appearance?: InputAppearance;
  source?: InputSource;
  size?: InputSize;
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
  appearance = "form",
  source = "default",
  size = "medium",
  className,
},
ref
) {


  const generatedId = useId();
  const inputId = id ?? generatedId;
  const unitWidth = unit ? `${Math.max(unit.length, 1)}ch` : undefined;
  const style = unitWidth
    ? ({ "--number-input-unit-width": unitWidth } as React.CSSProperties)
    : undefined;

  const isDisplayOnly = displayOnly;
  const isDisabled = disabled;
  const isReadOnly = readonly && !isDisabled;

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
  let raw = e.target.value;

  if (raw === "") {
    onChange?.("");
    return;
  }

  if (!INPUT_REGEX.test(raw)) {
    return;
  }

  if (raw.startsWith(".")) {
    raw = "0" + raw;
  }

  if (raw.startsWith("-.")) {
    raw = "-0" + raw.slice(1);
  }

  onChange?.(raw);
}


  return (
    <div
      className={clsx(
        "number-input",
        unit && "has-unit",
        isDisabled && "is-disabled",
        isReadOnly && "readonly",
        isDisplayOnly && "is-display-only",
        `number-input--${appearance}`,
        className
      )}
      style={style}
    >
      <InputBase
        wrapperClassName="ni-input-wrapper"
        rightSlot={
          unit ? (
            <span className="ni-unit">
              {unit}
            </span>
          ) : null
        }
          id={inputId}
          ref={ref}
          type="text"
          inputMode="decimal"
          pattern="-?[0-9]*[.,]?[0-9]*"

          autoComplete="off"
          name={`ni-${inputId}`}

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
          className={clsx(
            "ni-input",
            "input-control",
            `input-control--${appearance}`,
            `input-control--${size}`,
            source !== "default" && `input-control--${source}`,
            isDisabled && "input-control--disabled"
          )}
      />
    </div>
  );
});


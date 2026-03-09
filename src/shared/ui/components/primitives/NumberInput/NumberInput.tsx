import React, { useId } from "react";
import clsx from "clsx";
import type { FieldState } from "@shared/types/fields";
import {
  normalizeDecimalInput,
  safeParseDecimal,
} from "@shared/engine";
import "./NumberInput.css";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

type Props = {
  id?: string; // ← NY

  field: FieldState;
  onChange: (value: string) => void;

  unit?: string;

  disabled?: boolean;
  readonly?: boolean;

  autoFocus?: boolean;
  inputRef?: React.Ref<HTMLInputElement>;
  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;

  className?: string;
};

const INPUT_REGEX = /^-?\d*([.,]\d*)?$/;

export function NumberInput({
  id,
  field,
  onChange,
  unit,
  disabled = false,
  readonly = false,
  autoFocus,
  inputRef,
  onKeyDown,
  onFocus,
  onBlur,
  className,
}: Props) {
  const generatedId = useId();
  const inputId = id ?? generatedId;

  const isDisabled = disabled || field.locked;
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

  function handleBlurInternal(
    e: React.FocusEvent<HTMLInputElement>
  ) {
    if (isDisabled || isReadOnly) {
      onBlur?.(e);
      return;
    }

    const raw = field.value;

    if (!raw.trim()) {
      onBlur?.(e);
      return;
    }

    const normalized = normalizeDecimalInput(raw);
    const parsed = safeParseDecimal(normalized);

    if (parsed !== null) {
      onChange(parsed.toString());
    }

    onBlur?.(e);
  }

  const { decimals } = useDisplaySettings();

const displayValue =
  field.source === "machine" &&
  typeof field.machineValue === "number"
    ? field.machineValue.toFixed(decimals)
    : field.value ?? "";

  return (
    <div className={clsx("number-input", className)}>
      <div className="ni-input-wrapper">
        <input
          id={inputId}
          ref={inputRef}
          type="text"
          inputMode="decimal"
          pattern="-?[0-9]*[.,]?[0-9]*"
          autoFocus={autoFocus}
          value={isDisabled ? "" : displayValue}
          disabled={isDisabled}
          readOnly={isReadOnly}
          tabIndex={isReadOnly ? -1 : undefined}
          onChange={handleChange}
          onFocus={onFocus}
          onBlur={handleBlurInternal}
          onKeyDown={onKeyDown}
          className={clsx(
            "ni-input",
            `source-${field.source}`,
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
}

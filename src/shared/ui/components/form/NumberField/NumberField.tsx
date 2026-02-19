// shared/ui/components/form/NumberField/NumberField.tsx

import React, { useId } from "react";
import clsx from "clsx";
import { Field } from "../Field/Field";
import type { FieldState } from "@shared/types/fields";
import {
  normalizeDecimalInput,
  safeParseDecimal,
} from "@shared/engine";
import "./NumberField.css";

type Props = {
  label: string;
  field: FieldState;
  onChange: (value: string) => void;
  unit?: string;
  tooltip?: string;
  error?: string;

  /** Layout variant */
  variant?: "form" | "table";

  /** External hard disable */
  disabled?: boolean;

  /** UI-level readonly (execution mode etc.) */
  readonly?: boolean;

  autoFocus?: boolean;
  inputRef?: React.Ref<HTMLInputElement>;
  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
};

const INPUT_REGEX = /^-?\d*([.,]\d*)?$/;

export function NumberField({
  label,
  field,
  onChange,
  unit,
  tooltip,
  error,
  variant = "form",
  disabled = false,
  readonly = false,
  autoFocus,
  inputRef,
  onKeyDown,
  onFocus,
  onBlur,
}: Props) {
  const id = useId();

  /**
   * Domain-level disabled:
   * - constraint lock
   * - explicit disabled prop
   * MUST render empty
   */
  const isDisabled = disabled || field.locked;

  /**
   * UI-level readonly:
   * - execution mode
   * - solved display mode
   * Should show value but not allow editing
   */
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

  return (
    <Field
      label={variant === "table" ? "" : label}
      tooltip={variant === "table" ? undefined : tooltip}
      error={error}
      htmlFor={id}
    >
      <div
        className={clsx(
          "number-field",
          variant === "table" && "table"
        )}
      >
        <div className="nf-input-wrapper">
          <input
            id={id}
            ref={inputRef}
            type="text"
            inputMode="decimal"
            pattern="-?[0-9]*[.,]?[0-9]*"
            autoFocus={autoFocus}
            value={isDisabled ? "" : field.value}
            disabled={isDisabled}
            readOnly={isReadOnly}
            tabIndex={isReadOnly ? -1 : undefined}
            onChange={handleChange}
            onFocus={onFocus}
            onBlur={handleBlurInternal}
            onKeyDown={onKeyDown}
            className={clsx(
              "nf-input",
              `source-${field.source}`,
              isDisabled && "disabled",
              isReadOnly && "readonly"
            )}
          />

          {unit && (
            <span className="nf-unit">
              {unit}
            </span>
          )}
        </div>
      </div>
    </Field>
  );
}

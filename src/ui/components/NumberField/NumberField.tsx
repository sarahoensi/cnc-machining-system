// ui/components/NumberField/NumberField.tsx

import "./NumberField.css";
import React from "react";
import type { FieldState } from "@shared/types/fields";
import {
  normalizeDecimalInput,
  safeParseDecimal,
} from "@shared/engine";

type Props = {
  label: string;
  field: FieldState;
  onChange: (next: FieldState) => void;

  unit?: string;
  tooltip?: string;
  error?: string;

  disabled?: boolean;
  locked?: boolean;

  autoFocus?: boolean;
  inputRef?: React.Ref<HTMLInputElement>;

  onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>;
  onFocus?: React.FocusEventHandler<HTMLInputElement>;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
};

// Tillater:
// ""
// "-"
// "1"
// "1."
// "1,"
// "-1.23"
const INPUT_REGEX = /^-?\d*([.,]\d*)?$/;

export function NumberField({
  label,
  field,
  onChange,
  unit,
  tooltip,
  error,
  disabled = false,
  locked = false,
  autoFocus,
  inputRef,
  onKeyDown,
  onFocus,
  onBlur,
}: Props) {
  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    const raw = e.target.value;

    // Tillat tom streng
    if (raw === "") {
      onChange({ ...field, value: "" });
      return;
    }

    // Tillat midlertidig gyldig input
    if (INPUT_REGEX.test(raw)) {
      onChange({ ...field, value: raw });
    }
  }

  function handleBlurInternal(
    e: React.FocusEvent<HTMLInputElement>
  ) {
    const raw = field.value;

    if (!raw.trim()) {
      onBlur?.(e);
      return;
    }

    const normalized = normalizeDecimalInput(raw);
    const parsed = safeParseDecimal(normalized);

    if (parsed !== null) {
      // skriv tilbake normalisert verdi
      onChange({
        ...field,
        value: parsed.toString(),
      });
    }

    onBlur?.(e);
  }

  return (
    <div className="field number-field">
      <label className="nf-label">
        {tooltip ? <span title={tooltip}>{label}</span> : label}
      </label>

      <div className="nf-input-wrapper">
        <input
          ref={inputRef}
          type="text"
          inputMode="decimal"
          pattern="-?[0-9]*[.,]?[0-9]*"
          autoFocus={autoFocus}
          value={field.value}
          disabled={disabled}
          readOnly={locked}
          onChange={handleChange}
          onFocus={onFocus}
          onBlur={handleBlurInternal}
          onKeyDown={onKeyDown}
          className={[
            "nf-input",
            `source-${field.source}`,
            locked ? "locked" : "",
            disabled ? "disabled" : "",
            error ? "has-error" : "",
          ].join(" ")}
        />

        {unit && <span className="nf-unit">{unit}</span>}
      </div>

      {error && <div className="nf-error">{error}</div>}
    </div>
  );
}

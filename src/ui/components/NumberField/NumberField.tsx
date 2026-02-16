import "./NumberField.css";
import type { FieldState } from "@shared/types/fields";
import { handleNumericKeyDown } from "@shared/ui/behaviour/numericInputGuard";

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

  const readOnly = locked;

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    onChange({
      ...field,
      value: e.target.value,
    });
  }

  return (
    <div className="field number-field">
      <label className="nf-label">
        {tooltip ? (
          <span title={tooltip}>{label}</span>
        ) : (
          label
        )}
      </label>

      <div className="nf-input-wrapper">
        <input
          ref={inputRef}
          type="text"
          inputMode="decimal"
          autoFocus={autoFocus}
          value={field.value}
          disabled={disabled}
          readOnly={readOnly}

          onChange={handleChange}
          onFocus={onFocus}
          onBlur={onBlur}

          onKeyDown={(e) => {
            handleNumericKeyDown(e); // blokkerer ulovlige tegn
            onKeyDown?.(e);          // Enter-navigation osv.
          }}

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

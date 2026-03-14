// shared/ui/components/execution/ExecutionNumberField.tsx

import { forwardRef } from "react";
import { NumberInput } from "../primitives/NumberInput/NumberInput";
import type { ExecutionStepStatus } from "@shared/execution";
import "./ExecutionNumberField.css";

type Props = {
  state: ExecutionStepStatus;

  value?: string;
  placeholder?: string;
  unit?: string;
  error?: string;
  readonly?: boolean;

  autoFocus?: boolean;

  onChange?: (value: string) => void;
  onSubmit?: () => void;
};

export const ExecutionNumberField = forwardRef<
  HTMLInputElement,
  Props
>(function ExecutionNumberField(
  {
    state,
    value,
    placeholder,
    unit,
    error,
    readonly = false,
    autoFocus,
    onChange,
    onSubmit,
  },
  ref
) {


  /* -----------------------------------------------------------
     Pending step
     Future values should not be visible
  ----------------------------------------------------------- */

  if (state === "pending") {
    return <span></span>;
  }

  return (
    <div className="execution-number-field">
      <NumberInput
        ref={ref}
        value={value ?? ""}
        placeholder={placeholder}
        unit={unit}
        readonly={readonly || state === "completed"}
        autoFocus={autoFocus}
      
        onChange={(v) => onChange?.(v)}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            onSubmit?.();
          }
        }}
      />

      {error && (
        <div className="execution-number-error">
          {error}
        </div>
      )}
    </div>
  );
});
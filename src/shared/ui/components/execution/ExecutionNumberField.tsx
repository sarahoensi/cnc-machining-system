// shared/ui/components/execution/ExecutionNumberField.tsx

// shared/ui/components/execution/ExecutionNumberField.tsx

import { NumberInput } from "../primitives/NumberInput/NumberInput";
import type { ExecutionStepStatus } from "@shared/execution";
import "./ExecutionNumberField.css";

type Props = {
  state: ExecutionStepStatus;

  value?: string;
  placeholder?: string;
  unit?: string;
  error?: string
  readonly?: boolean;

  onChange?: (value: string) => void;
  onSubmit?: () => void;
};

export function ExecutionNumberField({
  state,
  value,
  placeholder,
  unit,
  error,
  readonly = false,
  onChange,
  onSubmit,
}: Props) {

  /* -----------------------------------------------------------
     Pending step
     Future values should not be visible
  ----------------------------------------------------------- */

  if (state === "pending") {
    return <span></span>;
  }

  /* -----------------------------------------------------------
     Active step
     Allow user input
  ----------------------------------------------------------- */

  return (
  <div className="execution-number-field">
    <NumberInput
      value={value ?? ""}
      placeholder={placeholder}
      unit={unit}
      readonly={readonly || state === "completed"}
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
}
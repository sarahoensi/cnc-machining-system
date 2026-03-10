// shared/ui/components/execution/ExecutionNumberField.tsx

// shared/ui/components/execution/ExecutionNumberField.tsx

import { NumberInput } from "../primitives/NumberInput/NumberInput";
import type { ExecutionStepStatus } from "@shared/execution";

type Props = {
  state: ExecutionStepStatus;

  value?: string;
  placeholder?: string;
  unit?: string;
  readonly?: boolean;

  onChange?: (value: string) => void;
  onSubmit?: () => void;
};

export function ExecutionNumberField({
  state,
  value,
  placeholder,
  unit,
  readonly = false,
  onChange,
  onSubmit,
}: Props) {

  /* -----------------------------------------------------------
     Pending step
     Future values should not be visible
  ----------------------------------------------------------- */

  if (state === "pending") {
    return <span>—</span>;
  }

  /* -----------------------------------------------------------
     Active step
     Allow user input
  ----------------------------------------------------------- */

  return (
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
  );
}
// shared/ui/components/execution/ExecutionInput.tsx

import { forwardRef } from "react";
import "./ExecutionField.css";
import { NumberInput } from "@shared/ui/primitives/input";

type Props = {
  value: string;
  placeholder?: string;
  unit?: string;
  error?: string;

  onChange: (value: string) => void;
  onSubmit?: () => void;
};

export const ExecutionInput = forwardRef<HTMLInputElement, Props>(
  function ExecutionInput(
    { value, placeholder, unit, error, onChange, onSubmit },
    ref,
  ) {
    return (
      <div className="exec-field">
        <NumberInput
          ref={ref}
          value={value}
          placeholder={placeholder}
          unit={unit}
          onChange={onChange}
          appearance="execution"
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              onSubmit?.();
            }
          }}
        />

        {error && <div className="exec-error">{error}</div>}
      </div>
    );
  },
);

// shared/ui/form/FormActions.tsx

/**
 * Form-level UI
 * Knows form-level actions, but not individual field behavior.
 */

import { Button } from "@shared/ui/primitives/Button/Button";
import clsx from "clsx";
import type { KeyboardEventHandler, Ref } from "react";

import "./FormActions.css";

type Props = {
  onCalculate: () => void;
  onReset: () => void;
  disabled?: boolean;
  children?: React.ReactNode;
  calculateRef?: Ref<HTMLButtonElement>;
  onCalculateKeyDown?: KeyboardEventHandler<HTMLButtonElement>;
  variant?: "default" | "inline";
};

export function FormActions({
  onCalculate,
  onReset,
  disabled,
  children,
  calculateRef,
  onCalculateKeyDown,
  variant = "default",
}: Props) {
  return (
    <div
      className={clsx(
        "form-actions",
        variant === "inline" && "form-actions--inline",
      )}
    >

      <div className="form-actions-primary">
        <Button
          ref={calculateRef}
          variant="primary"
          size="large"
          onClick={onCalculate}
          onKeyDown={onCalculateKeyDown}
          disabled={disabled}
        >
          Calculate
        </Button>
      </div>

      <div className="form-actions-secondary">
        
        {children}
        <Button variant="danger" size="medium" onClick={onReset}>
          Clear form
        </Button>
      </div>

    </div>
  );
}

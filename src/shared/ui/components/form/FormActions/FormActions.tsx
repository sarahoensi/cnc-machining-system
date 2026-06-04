// shared/ui/components/form/FormActions/FormActions.tsx

/**
 * Form-level UI
 * Vet hva som kan gjøres med hele formatet.
 * Kjenner ikke enkeltfelter
 */

import {
  CalculateButton,
  ResetButton,
} from "@shared/ui/primitives/Button/Button";
import type { KeyboardEventHandler, Ref } from "react";

import "./FormActions.css";

type Props = {
  onCalculate: () => void;
  onReset: () => void;
  disabled?: boolean;
  children?: React.ReactNode;
  calculateRef?: Ref<HTMLButtonElement>;
  onCalculateKeyDown?: KeyboardEventHandler<HTMLButtonElement>;
};

export function FormActions({
  onCalculate,
  onReset,
  disabled,
  children,
  calculateRef,
  onCalculateKeyDown,
}: Props) {
  return (
    <div className="form-actions">

      <div className="form-actions-primary">
        <CalculateButton
          ref={calculateRef}
          onClick={onCalculate}
          onKeyDown={onCalculateKeyDown}
          disabled={disabled}
        />
      </div>

      <div className="form-actions-secondary">
        
        {children}
        <ResetButton onClick={onReset} />
      </div>

    </div>
  );
}

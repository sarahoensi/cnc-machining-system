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

import "./FormActions.css";

type Props = {
  onCalculate: () => void;
  onReset: () => void;
  disabled?: boolean;
  children?: React.ReactNode;
};

export function FormActions({
  onCalculate,
  onReset,
  disabled,
  children,
}: Props) {
  return (
    <div className="form-actions">

      <div className="form-actions-primary">
        <CalculateButton
          onClick={onCalculate}
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
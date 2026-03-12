// shared/ui/components/form/FormActions/FormActions.tsx

import {
  CalculateButton,
  ResetButton,
} from "@shared/ui/components/primitives/Button/Button";

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
      <CalculateButton
        onClick={onCalculate}
        disabled={disabled}
      />

      <ResetButton onClick={onReset} />

      {children}
    </div>
  );
}
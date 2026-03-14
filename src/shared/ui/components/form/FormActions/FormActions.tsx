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
      <div className="form-actions-primary">
        <CalculateButton
          onClick={onCalculate}
          disabled={disabled}
        />
      </div>

      <div className="form-actions-secondary">
        <ResetButton onClick={onReset} />
      </div>

      {children}
    </div>
  );
}
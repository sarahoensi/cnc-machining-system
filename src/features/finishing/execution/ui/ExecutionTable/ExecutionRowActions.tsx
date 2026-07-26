// finishing/ui/execution/ExecutionRowActions.tsx

import { Button } from "@shared/ui/primitives/Button/Button";

type Props = {
  stepIndex: number;

  isEditing: boolean;
  isActive: boolean;
  isEditableCompleted: boolean;
  finished: boolean;

  value: string;

  measurementValue?: string;

  onConfirm(step: number): void;
  onStartEdit(step: number, value: string): void;
  onCancelEdit(): void;
};

export function ExecutionRowActions({
  stepIndex,
  isEditing,
  isActive,
  isEditableCompleted,
  value,
  measurementValue,
  finished,
  onConfirm,
  onStartEdit,
  onCancelEdit,
}: Props) {

  if (finished) {
  return null
}

  if (isEditing) {
    return (
      <>
        <Button variant="primary" size="small" onClick={() => onConfirm(stepIndex)}>
          OK
        </Button>
        <Button variant="secondary" size="small" onClick={onCancelEdit}>
          Cancel
        </Button>
      </>
    );
  }

  if (isActive) {
    return (
      <Button
        variant="primary"
        size="small"
        disabled={!value}
        onClick={() => onConfirm(stepIndex)}
      >
        Registrer
      </Button>
    );
  }

  if (isEditableCompleted) {
    return (
      <Button
        variant="secondary"
        size="small"
        onClick={() =>
          onStartEdit(stepIndex, measurementValue ?? "")
        }
      >
        Edit
      </Button>
    );
  }

  return null;
}

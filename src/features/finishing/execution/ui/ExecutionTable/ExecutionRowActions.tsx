// finishing/ui/execution/ExecutionRowActions.tsx

import {
  CancelButton,
  OkButton,
  RegisterButton,
  EditButton,
} from "@shared/ui/primitives/Button/Button";

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
        <OkButton onClick={() => onConfirm(stepIndex)} />
        <CancelButton onClick={onCancelEdit} />
      </>
    );
  }

  if (isActive) {
    return (
      <RegisterButton
        disabled={!value}
        onClick={() => onConfirm(stepIndex)}
      />
    );
  }

  if (isEditableCompleted) {
    return (
      <EditButton
        onClick={() =>
          onStartEdit(stepIndex, measurementValue ?? "")
        }
      >
        Edit
      </EditButton>
    );
  }

  return null;
}
// features/finishing/ui/plan/PlanForm.tsx

import {
  handleUserEdit,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import {
  EditButton,
} from "@shared/ui/components/primitives/Button/Button";

import { useFormNavigation } from "@shared/ui";

import {
  FinishingKey,
  createInitialFinishingForm,
} from "../../domain/plan/finishingForm";

import {
  validFinishingInputSets,
  mutuallyExclusiveFinishingPairs,
} from "../../domain/plan/finishingConstraints";

import { finishingFieldConfig } from "./finishingFieldConfig";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";

type Props = {
  form: ReturnType<typeof createInitialFinishingForm>;
  setForm: (v: any) => void;
  onGenerate: () => void;
  onReset: () => void;
  onEdit: () => void;
  readOnly: boolean;
};

export function PlanForm({
  form,
  setForm,
  onGenerate,
  onReset,
  onEdit,
  readOnly,
}: Props) {

  const navigation = useFormNavigation({
    keys: finishingFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    onSubmit: onGenerate,
  });

  function onFieldChange(
    key: FinishingKey,
    value: string
  ) {
    setForm((prev: any) =>
      handleUserEdit(
        prev,
        key,
        value,
        validFinishingInputSets,
        mutuallyExclusiveFinishingPairs
      )
    );
  }


  return (
    <>
      <ModeSelector
        name="finishing-mode"
        label="Mode"
        value={form.extras.mode}
        readonly={readOnly}
        onChange={(newMode) =>
          setForm((prev: any) =>
            handleModeChange(prev, {
              ...prev.extras,
              mode: newMode,
            })
          )
        }
        options={[
          { value: "Inner", label: "Inner" },
          { value: "Outer", label: "Outer" },

        ]}
      />

      {finishingFieldConfig.map((f) => {

        const fieldState = form.fields[f.key];

        return (
          <FormNumberField
            key={f.key}
            label={f.label}
            unit={f.unit}
            field={fieldState}
            disabled={fieldState.locked || f.readOnly}
            readonly={readOnly}
            error={fieldState.error}
            autoFocus={f.autoFocus}
            onChange={(value) =>
              onFieldChange(f.key, value)
            }
            inputRef={navigation.register(f.key)}
            onKeyDown={navigation.handleKeyDown(f.key)}
          />
        );
      })}


      <FormActions
        onCalculate={onGenerate}
        onReset={onReset}
        disabled={readOnly}
      >
        {readOnly && (
          <EditButton onClick={onEdit}>
            Edit plan
          </EditButton>
        )}
      </FormActions>



    </>
  );
}
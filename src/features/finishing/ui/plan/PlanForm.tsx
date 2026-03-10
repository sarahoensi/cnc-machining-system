// features/finishing/ui/plan/PlanForm.tsx

import {
  handleUserEdit,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import {
  CalculateButton,
  ResetButton,
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

type Props = {
  form: ReturnType<typeof createInitialFinishingForm>;
  setForm: (v: any) => void;
  onGenerate: () => void;
  onReset: () => void;
};

export function PlanForm({
  form,
  setForm,
  onGenerate,
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

  function onReset() {
    setForm(createInitialFinishingForm());
  }

  return (
    <>

      <div style={{ marginBottom: 16 }}>
        <ModeSelector
          name="finishing-mode"
          label="Mode"
          value={form.extras.mode}
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
      </div>

      {finishingFieldConfig.map((f) => {

        const fieldState = form.fields[f.key];

        return (
          <FormNumberField
            key={f.key}
            label={f.label}
            unit={f.unit}
            field={fieldState}
            disabled={fieldState.locked || f.readOnly}
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

      <div style={{ marginTop: 16, display: "flex", gap: 12 }}>
        <CalculateButton onClick={onGenerate} />
        <ResetButton onClick={onReset} />
      </div>

    </>
  );
}
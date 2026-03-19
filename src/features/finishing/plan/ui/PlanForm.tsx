// features/finishing/ui/plan/PlanForm.tsx

import {
  handleUserEdit,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";

import { useFormNavigation } from "@shared/ui";

import { finishingFieldConfig } from "./finishingFieldConfig";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { createInitialFinishingForm, FinishingKey } from "../domain/finishingForm";
import { mutuallyExclusiveFinishingPairs, validFinishingInputSets } from "../domain/finishingConstraints";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";

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

  const fields = (
    <>
      <FormModeField
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
            autoFocus={f.autoFocus}
            onChange={(value) =>
              onFieldChange(f.key, value)
            }
            ref={navigation.register(f.key)}
            onKeyDown={navigation.handleKeyDown(f.key)}
          />
        );
      })}
    </>
  );

  const error = form.formError ? (
    <FormError error={form.formError} />
  ) : null;

  const actions = (
    <FormActions
      onCalculate={onGenerate}
      onReset={onReset}
    />
  );

  return (
    <FormLayout
      fields={fields}
      error={error}
      actions={actions}
    />
  );
}
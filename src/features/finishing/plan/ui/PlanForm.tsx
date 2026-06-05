// features/finishing/ui/plan/PlanForm.tsx

import {
  handleUserEdit,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { FormModeField } from "@shared/ui/form/fields/FormModeField";

import { useFormNavigation } from "@shared/hooks";

import { finishingFieldConfig } from "./finishingFieldConfig";
import { FormActions } from "@shared/ui/form/FormActions";
import { createInitialFinishingForm, FinishingKey } from "../domain/finishingForm";
import { mutuallyExclusiveFinishingPairs, validFinishingInputSets } from "../domain/finishingConstraints";
import { FormError } from "@shared/ui/form/FormError";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { Stack } from "@shared/ui/primitives/Stack/Stack";
import { finishingTooltips } from "./finishingPlanTooltip";

type Props = {
  form: ReturnType<typeof createInitialFinishingForm>;
  setForm: (v: any) => void;
  onGenerate: () => Promise<ReturnType<typeof createInitialFinishingForm> | void> | ReturnType<typeof createInitialFinishingForm> | void;
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
    activePath: "/finishing",
    onSubmit: onCalculate,
  });
  const focusOrder = finishingFieldConfig.map((f) => f.key);

  async function onCalculate() {
    const next = await onGenerate();
    if (!next) return;
    const hasInlineError = finishingFieldConfig.some((f) => Boolean(next.fields[f.key].error));

    if (hasInlineError) {
      navigation.focusFirstInvalidAfterRender((key) => Boolean(next.fields[key].error));
      return;
    }

    if (!next.formError) return;

    navigation.focusFirstInOrderAfterRender(focusOrder, (key) => {
      const value = next.fields[key]?.value;
      return value == null || String(value).trim() === "";
    });
  }

  function handleReset() {
    onReset();
    navigation.focusFirstAfterRender();
  }

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
      <Stack className="stack--form-section">
        <FormModeField
          label="Mode"
          tooltip={finishingTooltips.mode}
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
      </Stack>

      <Stack className="stack--form-section">
        {finishingFieldConfig.map((f) => {
          const fieldState = form.fields[f.key];

          return (
            <FormNumberField
              key={f.key}
              label={f.label}
              unit={f.unit}
              tooltip={f.tooltip}
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
      </Stack>
    </>
  );

  const error = form.formError ? (
    <FormError error={form.formError} />
  ) : null;

  const actions = (
    <FormActions
      onCalculate={onCalculate}
      onReset={handleReset}
    />
  );

  return (
    <div ref={navigation.containerRef}>
      <FormLayout
        error={error}
        actions={actions}
      >
        {fields}
      </FormLayout>
    </div>
  );
}


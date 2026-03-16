// features/helix/ui/HelixPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";

import { useFormNavigation } from "@shared/ui";

import {
  createInitialHelixForm,
  HelixKey,
} from "../domain/helixForm";

import { parseHelix } from "../domain/parseHelix";
import { solveHelix } from "../api/solveHelix";
import { helixFieldConfig } from "./helixFieldConfig";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";

import {
  validHelixInputSets,
  mutuallyExclusiveHelixPairs,
} from "../domain/helixConstraints"

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormFigureLayout } from "@shared/ui/layout/FormFigureLayout/FormFigureLayout";
/* ============================================================
   Component
============================================================ */

export function HelixPage() {

  usePageTitle("Helix");

  const [form, setForm] = useFeatureForm(
    "helix",
    createInitialHelixForm
  );

  const navigation = useFormNavigation({
    keys: helixFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    onSubmit: onCalculate,
  });

  /* =========================
     Field change
  ========================= */

  function onFieldChange(
    key: HelixKey,
    value: string
  ) {
    setForm(prev =>
      handleUserEdit(prev,
        key,
        value,
        validHelixInputSets,
        mutuallyExclusiveHelixPairs)
    );
  }

  /* =========================
     Calculate
  ========================= */

  async function onCalculate() {

    const next = await handleCalculateAsync(
      form,
      parseHelix,
      (input) => solveHelix(input, form.extras.mode)
    );

    setForm(next);
  }

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialHelixForm());
  }

  /* =========================
     Render
  ========================= */

  const formContent = (
    <>
      <div>
        <ModeSelector
          name="helix-mode"
          label="Mode"
          value={form.extras.mode}
          onChange={(newMode) =>
            setForm(prev =>
              handleModeChange(prev, {
                ...prev.extras,
                mode: newMode,
              })
            )
          }
          options={[
            { value: "Outer", label: "Outer" },
            { value: "Inner", label: "Inner" },
          ]}
        />
      </div>

      {helixFieldConfig.map((f) => {
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
            ref={navigation.register(f.key)}
            onKeyDown={navigation.handleKeyDown(f.key)}
          />
        );
      })}

      <FormActions
        onCalculate={onCalculate}
        onReset={onReset}
      />
    </>
  );


  return (
    <FormFigureLayout
      form={formContent}
      figure={null}
    />
  );
}
// features/helix/ui/HelixPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { CalculatorNumberFields } from "@shared/ui/components/form/fields";

import { useFormNavigation } from "@shared/ui";
import { validateHelixForm } from "../domain/validateHelixForm";

import {
  createInitialHelixForm,
  HelixKey,
} from "../domain/helixForm";

import { parseHelix } from "../domain/parseHelix";
import { solveHelix } from "../api/solveHelix";
import { helixFieldConfig } from "./helixFieldConfig";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";

import {
  validHelixInputSets,
  mutuallyExclusiveHelixPairs,
} from "../domain/helixConstraints"

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormFigureLayout } from "@shared/ui/layout/page/FormFigureLayout/FormFigureLayout";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { helixTooltips } from "./helixTooltip";
import { HelixFigure } from "./helixFigure/HelixFigure";
import { useState } from "react";
/* ============================================================
   Component
============================================================ */

export function HelixPage() {

  usePageTitle("Helix");

  const [form, setForm] = useFeatureForm(
    "helix",
    createInitialHelixForm
  );

  const [activeField, setActiveField] = useState<HelixKey | null>(null);

  const navigation = useFormNavigation({
    keys: helixFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    activePath: "/helix",
    onSubmit: onCalculate,
  });
  const focusOrder = helixFieldConfig.map((f) => f.key);

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
     Mode change
  ========================= */

  function onModeChange(newMode: typeof form.extras.mode) {
    setForm(prev =>
      handleModeChange(prev, {
        ...prev.extras,
        mode: newMode,
      })
    );
  }

  /* =========================
     Calculate
  ========================= */

  async function onCalculate() {

    const next = await handleCalculateAsync(
      form,
      parseHelix,
      (input) => solveHelix(input, form.extras.mode),
      validateHelixForm,
    );

    setForm(next);
    const hasInlineError = helixFieldConfig.some((f) => Boolean(next.fields[f.key].error));

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

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialHelixForm());
    navigation.focusFirstAfterRender();
  }

  /* =========================
     Render
  ========================= */

  const fields = (
  <>
    <FormSection>
      <FormModeField
        label="Mode"
        tooltip={helixTooltips.mode}
        value={form.extras.mode}
        onChange={onModeChange}
        options={[
          { value: "Outer", label: "Outer" },
          { value: "Inner", label: "Inner" },
        ]}
      />
    </FormSection>

    <FormSection>
      <CalculatorNumberFields
        configs={helixFieldConfig}
        fields={form.fields}
        onChange={onFieldChange}
        register={navigation.register}
        onKeyDown={navigation.handleKeyDown}
        onFocus={setActiveField}
        onBlur={() => setActiveField(null)}
      />
    </FormSection>
  </>
);

const error = form.formError ? (
  <FormError error={form.formError} />
) : null;

const actions = (
  <FormActions
    onCalculate={onCalculate}
    onReset={onReset}
  />
);

const formContent = (
  <div ref={navigation.containerRef}>
    <FormLayout
      fields={fields}
      error={error}
      actions={actions}
    />
  </div>
);


  return (
  <FormFigureLayout
    form={formContent}
    figure={
      <HelixFigure
        mode={form.extras.mode}
        activeField={activeField}
      />
    }
  />
);
}

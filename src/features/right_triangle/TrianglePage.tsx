// features/right_triangle/TrianglePage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { CalculatorNumberFields } from "@shared/ui/components/form/fields";

import {
  createInitialTriangleForm,
  TriangleKey,
} from "./domain/triangleForm";

import { parseTriangle } from "./domain/parseTriangle";
import { solveTriangle } from "./api/solveTriangle";
import { triangleFieldConfig } from "./ui/triangleFieldConfig";


import { useFormNavigation } from "@shared/ui";

import {
  validTriangleInputSets,
  mutuallyExclusiveTrianglePairs,
} from "./domain/triangleConstraints";

import { useFeatureForm } from "@app/providers/FormStateProvider";

import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";

import { FormFigureLayout } from "@shared/ui/layout/page/FormFigureLayout/FormFigureLayout";
import { FormError } from "@shared/ui/components/form/FormError/FormError";

import { validateTriangleForm } from "./domain/validateTriangleForm";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { useState } from "react";
import { TriangleFigure } from "./ui/triangleFigure/TriangleFigure";

export function TrianglePage() {

  usePageTitle("Triangle");

  const [form, setForm] = useFeatureForm(
    "triangle",
    createInitialTriangleForm
  );

  const [activeField, setActiveField] = useState<TriangleKey | null>(null);

  const navigation = useFormNavigation({
    keys: triangleFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    activePath: "/triangle",
    onSubmit: onCalculate,
  });
  const focusOrder = triangleFieldConfig.map((f) => f.key);

  /* =========================
     Field change
  ========================= */

  function onFieldChange(
    key: TriangleKey,
    value: string
  ) {
    setForm(prev =>
      handleUserEdit(
        prev,
        key,
        value,
        validTriangleInputSets,
        mutuallyExclusiveTrianglePairs
      )
    );
  }

  /* =========================
     Calculate
  ========================= */

  async function onCalculate() {

    const next = await handleCalculateAsync(
      form,
      parseTriangle,
      solveTriangle,
      validateTriangleForm,
    );

    setForm(next);
    const hasInlineError = triangleFieldConfig.some((f) => Boolean(next.fields[f.key].error));

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
    setForm(createInitialTriangleForm());
    navigation.focusFirstAfterRender();
  }

  /* =========================
     Render
  ========================= */

  const fields = (
    <CalculatorNumberFields
      configs={triangleFieldConfig}
      fields={form.fields}
      onChange={onFieldChange}
      register={navigation.register}
      onKeyDown={navigation.handleKeyDown}
      onFocus={setActiveField}
      onBlur={() => setActiveField(null)}
    />
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
    <TriangleFigure activeField={activeField} />
  }
/>
);
  
}

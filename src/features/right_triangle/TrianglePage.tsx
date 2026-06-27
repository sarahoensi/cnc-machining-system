// features/right_triangle/TrianglePage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import {
  createInitialTriangleForm,
  TriangleKey,
} from "./domain/triangleForm";

import { parseTriangle } from "./domain/parseTriangle";
import { solveTriangle } from "./api/solveTriangle";
import { triangleFieldConfig } from "./ui/triangleFieldConfig";


import { useCalculatorFormNavigation } from "@shared/hooks";

import {
  validTriangleInputSets,
  mutuallyExclusiveTrianglePairs,
} from "./domain/triangleConstraints";

import { useFeatureForm } from "@app/providers/FormStateProvider";

import { FormActions } from "@shared/ui/form/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";

import { FormError } from "@shared/ui/form/FormError";

import { validateTriangleForm } from "./domain/validateTriangleForm";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";
import { TriangleFigure } from "./ui/triangleFigure/TriangleFigure";

export function TrianglePage() {

  usePageTitle("Triangle");

  const [form, setForm] = useFeatureForm(
    "triangle",
    createInitialTriangleForm
  );

  const fieldOrder = triangleFieldConfig.map((f) => f.key);
  const formFocus = useCalculatorFormNavigation({
    fieldOrder,
    activePath: "/triangle",
    onSubmit: onCalculate,
    trackActiveField: true,
  });

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
    formFocus.focusAfterCalculate(next);
  }

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialTriangleForm());
    formFocus.focusAfterReset();
  }

  /* =========================
     Render
  ========================= */

  const fields = (
    <FormNumberFields
      configs={triangleFieldConfig}
      fields={form.fields}
      onChange={onFieldChange}
      register={formFocus.register}
      onKeyDown={formFocus.handleKeyDown}
      onFocusField={formFocus.onFieldFocus}
      onBlurFields={formFocus.onFieldBlur}
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
  <div ref={formFocus.containerRef}>
    <FormLayout
      error={error}
      actions={actions}
    >
      {fields}
    </FormLayout>
  </div>
);



  return (
    <FormWithSidePanel
      form={formContent}
      sidePanel={<TriangleFigure activeField={formFocus.activeField} />}
    />
  );
}


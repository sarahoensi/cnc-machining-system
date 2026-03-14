// features/right_triangle/TrianglePage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";

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
import { Page } from "@app/shell/Page";


export function TrianglePage() {

  const [form, setForm] = useFeatureForm(
    "triangle",
    createInitialTriangleForm
  );

  const navigation = useFormNavigation({
    keys: triangleFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    onSubmit: onCalculate,
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
      solveTriangle
    );

    setForm(next);
  }

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialTriangleForm());
  }

  /* =========================
     Render
  ========================= */

  return (
    <Page title="Triangle">
    <div className="app-content split">

      <div className="app-left">

        {triangleFieldConfig.map((f) => {
          const fieldState = form.fields[f.key];

          return (
            <FormNumberField
              key={f.key}
              label={f.label}
              tooltip={f.tooltip}
              unit={f.unit}
              field={fieldState}
              disabled={fieldState.locked}
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



      </div>

      <div className="app-right">
        {/* <TriangleFigure form={form} /> */}
      </div>

    </div>
    </Page>
  );
  
}
// features/right_triangle/TrianglePage.tsx

import { useState } from "react";
import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";

import {
  createInitialTriangleForm,
  TriangleKey,
} from "./domain/triangleForm";

import { parseTriangle } from "./domain/parseTriangle";
import { solveTriangle } from "./api/solveTriangle";
import { triangleFieldConfig } from "./ui/triangleFieldConfig";

import {
  CalculateButton,
  ResetButton,
} from "@shared/ui/components/primitives/Button/Button";
import { useFormNavigation } from "@shared/ui";

import {
  validTriangleInputSets,
  mutuallyExclusiveTrianglePairs,
} from "./domain/triangleConstraints";

export function TrianglePage() {

  const [form, setForm] = useState(createInitialTriangleForm());

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
              error={fieldState.invalid ? "Ugyldig verdi" : undefined}
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

          <CalculateButton
            onClick={onCalculate}
            disabled={form.status === "executing"}
          />

          <ResetButton onClick={onReset} />

        </div>

      </div>

      <div className="app-right">
        {/* <TriangleFigure form={form} /> */}
      </div>

    </div>
  );
}
// features/right_triangle/TrianglePage.tsx

import { useState } from "react";
import {
  handleUserEdit,
  handleCalculate,
} from "@shared/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";

import {
  createInitialTriangleForm,
  TriangleKey,
} from "./triangleForm";

import { parseTriangle } from "./parseTriangle";
import { solveTriangle } from "./solveTriangle";

import { triangleFieldConfig } from "./triangleFieldConfig";

import { CalculateButton, ResetButton } from "@shared/ui/components/primitives/Button/Button";

/* ============================================================
   Constraint definitions
============================================================ */

const validSets = [
  ["a", "b"],
  ["a", "alpha"],
  ["a", "beta"],
  ["b", "beta"],
  ["b", "alpha"],
  ["c", "alpha"],
  ["c", "beta"],
  ["c", "a"],
  ["c", "b"]
] as const;

const pairs = [
  ["alpha", "beta"],
] as const;

/* ============================================================
   Component
============================================================ */

export function TrianglePage() {

  const [form, setForm] =
    useState(createInitialTriangleForm());

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
        validSets,
        pairs
      )
    );
  }

  /* =========================
     Calculate
  ========================= */

  function onCalculate() {
    setForm(prev =>
      handleCalculate(
        prev,
        parseTriangle,
        solveTriangle
      )
    );
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

      {/* LEFT SIDE */}
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
            />
          );
        })}

        <div style={{ marginTop: 16, display: "flex", gap: 12 }}>

          <CalculateButton
            onClick={onCalculate}
            disabled={form.status === "executing"}
          />

          <ResetButton
            onClick={onReset}
          />

        </div>

      </div>

      {/* RIGHT SIDE */}
      <div className="app-right">
        {/* <TriangleFigure form={form} /> */}
      </div>

    </div>
  );
}
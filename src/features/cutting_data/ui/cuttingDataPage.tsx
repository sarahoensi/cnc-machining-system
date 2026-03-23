// features/cuttingData/ui/CuttingDataPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { useFormNavigation } from "@shared/ui";

import {
  createInitialCuttingDataForm,
  CuttingDataKey,
} from "../domain/cuttingDataForm";

import { parseCuttingData } from "../domain/parseCuttingData";
import { solveCuttingData } from "../api/solveCuttingData";
import { cuttingDataFieldConfig } from "./cuttingDataFieldConfig";

import {
  mutuallyExclusiveCuttingDataPairs,
  validCuttingDataInputSets,
} from "../domain/cuttingDataConstraints";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormFigureLayout } from "@shared/ui/layout/page/FormFigureLayout/FormFigureLayout";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { validateCuttingDataForm } from "../domain/validateCuttingForm";

import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { useState } from "react";
import { CuttingHistoryPanel } from "./CuttingHistoryPanel";
import { Button } from "@shared/ui/primitives/Button/Button";

/* ============================================================
   Types
============================================================ */

type SavedEntry = {
  id: string;
  form: ReturnType<typeof createInitialCuttingDataForm>;
  createdAt: number;
};

/* ============================================================
   Component
============================================================ */

export function CuttingDataPage() {

  usePageTitle("Cutting Data");

const [form, setForm] = useFeatureForm(
  "cutting",
  createInitialCuttingDataForm
);

const [history, setHistory] = useState<SavedEntry[]>([]);

  const navigation = useFormNavigation({
    keys: cuttingDataFieldConfig.map(f => f.key),
    autoFocusOnMount: true,
    onSubmit: onCalculate,
  });

  /* =========================
     Field change
  ========================= */

  function onFieldChange(
    key: CuttingDataKey,
    value: string
  ) {
    setForm(prev =>
      handleUserEdit(
        prev,
        key,
        value,
        validCuttingDataInputSets,
        mutuallyExclusiveCuttingDataPairs
      )
    );
  }

  /* =========================
     Calculate
  ========================= */

  async function onCalculate() {

    const next = await handleCalculateAsync(
      form,
      parseCuttingData,
      (input) => solveCuttingData(input),
      validateCuttingDataForm,
    );

    setForm(next);
  }

  /* =========================
     Save / Load / Delete
  ========================= */

  function onSave() {
    if (form.status !== "solved") return;

    setHistory((prev) => [
      ...prev,
      {
        id: crypto.randomUUID(),
        form: structuredClone(form),
        createdAt: Date.now(),
      },
      
    ]);
  }

  function onLoad(entry: SavedEntry) {
    const cloned = structuredClone(entry.form);

    setForm({
      ...cloned,
      status: "solved", // sikrer konsistens
    });
  }

  function onDelete(id: string) {
    setHistory((prev) => prev.filter((e) => e.id !== id));
  }

  function onClearHistory() {
    setHistory([]);
  }

  function onReset() {
    setForm(createInitialCuttingDataForm());
  }

  /* =========================
     Fields
  ========================= */

  const fields = (
    <>
      {cuttingDataFieldConfig.map((f) => {
        const fieldState = form.fields[f.key];

        return (
          <FormNumberField
            key={f.key}
            label={f.label}
            unit={f.unit}
            tooltip={f.tooltip}
            field={fieldState}
            disabled={fieldState.locked || f.readOnly}
            autoFocus={f.autoFocus}
            onChange={(value) => onFieldChange(f.key, value)}
            ref={navigation.register(f.key)}
            onKeyDown={navigation.handleKeyDown(f.key)}
          />
        );
      })}
    </>
  );


  /* =========================
     UI blocks
  ========================= */


const error = form.formError ? (
  <FormError error={form.formError} />
) : null;

const actions = (
  <FormActions
    onCalculate={onCalculate}
    onReset={onReset}
  />
    
);




const saveButton =
  form.status === "solved" ? (
    <div className="form-save-row">
      <Button
        variant="secondary"
        size="small"
        onClick={onSave}
      >
        Save result
      </Button>
    </div>
  ) : null;

  const formContent = (
  <div className="cutting-form">
    <FormLayout
      fields={
        <>
          {fields}
          {saveButton}   {/* ← HER */}
        </>
      }
      error={error}
      actions={actions}
    />
  </div>
);

  /* =========================
     Render
  ========================= */

return (
  <FormFigureLayout
    form={formContent}
   figure={
        <CuttingHistoryPanel
          history={history}
          onLoad={onLoad}
          onDelete={onDelete}
          onClear={onClearHistory}
        />
      }
  />
);
}
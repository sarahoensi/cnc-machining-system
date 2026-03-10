// features/cuttingData/ui/CuttingDataPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";
import {
  CalculateButton,
  ResetButton,
} from "@shared/ui/components/primitives/Button/Button";

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


/* ============================================================
   Component
============================================================ */

export function CuttingDataPage() {

const [form, setForm] = useFeatureForm(
  "cutting",
  createInitialCuttingDataForm
);

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
      (input) => solveCuttingData(input)
    );

    setForm(next);
  }

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialCuttingDataForm());
  }

  /* =========================
     Render
  ========================= */

  return (
    <div className="app-content split">

      <div className="app-left">

        {cuttingDataFieldConfig.map((f) => {
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
        {/* Future cutting data visualization */}
      </div>

    </div>
  );
}
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
import { FormFigureLayout } from "@shared/ui/layout/FormFigureLayout/FormFigureLayout";

/* ============================================================
   Component
============================================================ */

export function CuttingDataPage() {

  usePageTitle("Cutting Data");

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


  const formContent = (
  <>
    {cuttingDataFieldConfig.map((f) => {
      const fieldState = form.fields[f.key];

      return (
        <FormNumberField
          key={f.key}
          label={f.label}
          unit={f.unit}
          field={fieldState}
          disabled={fieldState.locked || f.readOnly}
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

  /* =========================
     Render
  ========================= */

return (
  <FormFigureLayout
    form={formContent}
    figure={null}
  />
);
}
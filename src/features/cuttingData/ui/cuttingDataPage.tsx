// features/cuttingData/ui/CuttingDataPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { useFormNavigation } from "@shared/ui";

import {
  CuttingDataKey,
} from "../domain/cuttingDataForm";

import { parseCuttingData } from "../domain/parseCuttingData";
import { solveCuttingData } from "../api/solveCuttingData";
import { cuttingDataFieldConfig } from "./cuttingDataFieldConfig";

import {
  mutuallyExclusiveCuttingDataPairs,
  validCuttingDataInputSets,
} from "../domain/cuttingDataConstraints";

import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormSidebarLayout } from "@shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { validateCuttingDataForm } from "../domain/validateCuttingForm";

import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { CuttingHistoryPanel } from "./history/CuttingHistoryPanel";
import { useCuttingPageController } from "./useCuttingPageController";
import { Button } from "@shared/ui/primitives/Button/Button";
import "./cuttingDataPage.css";



/* ============================================================
   Component
============================================================ */

export function CuttingDataPage() {

  usePageTitle("Cutting Data");

 const {
    form,
    setForm,
    history,
    save,
    load,
    remove,
    clear,
    resetForm,
  } = useCuttingPageController();

  const navigation = useFormNavigation({
    keys: cuttingDataFieldConfig.map((f) => f.key),
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

  const saveButton =
 (
    <Button
      variant="secondary"
      size="medium"
      onClick={save}
    >
      Save result
    </Button>
  );


 const actions = (
    <FormActions
      onCalculate={onCalculate}
      onReset={resetForm}
    >
    {saveButton}
  </FormActions>
  );



  const formContent = (
    <div className="cutting-form">
      <FormLayout
        fields={fields}
        error={error}
        actions={actions}
      />
    </div>
  );

  /* =========================
     Render
  ========================= */

return (
  <FormSidebarLayout
      className="cutting-data-layout"
      form={formContent}
      sidebar={
        <CuttingHistoryPanel
          history={history}
          onLoad={load}
          onDelete={remove}
          onClear={clear}
        />
      }
  />
);
}

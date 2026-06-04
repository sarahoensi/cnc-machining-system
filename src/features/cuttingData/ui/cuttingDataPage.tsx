// features/cuttingData/ui/CuttingDataPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { CalculatorNumberFields } from "@shared/ui/components/form/fields";
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
    activePath: "/cutting",
    onSubmit: onCalculate,
  });
  const focusOrder: CuttingDataKey[] = [
    "diameter",
    "rpm",
    "cutting_speed",
    "teeth",
    "feed_rate",
    "chip_load",
  ];
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
    const hasInlineError = focusOrder.some((key) => Boolean(next.fields[key].error));

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

  function onReset() {
    resetForm();
    navigation.focusFirstAfterRender();
  }


  /* =========================
     Fields
  ========================= */

  const fields = (
    <CalculatorNumberFields
      configs={cuttingDataFieldConfig}
      fields={form.fields}
      onChange={onFieldChange}
      register={navigation.register}
      onKeyDown={navigation.handleKeyDown}
    />
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
      onReset={onReset}
    >
    {saveButton}
  </FormActions>
  );



 const formContent = (
    <FormLayout
      fields={fields}
      error={error}
      actions={actions}
      actionsPlacement="bottom"
      containerRef={navigation.containerRef}
    />
  );

  /* =========================
     Render
  ========================= */

return (
  <FormSidebarLayout
      fillHeight
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

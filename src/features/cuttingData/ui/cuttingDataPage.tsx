// features/cuttingData/ui/CuttingDataPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";

import { useFormNavigation } from "@shared/hooks";

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

import { FormActions } from "@shared/ui/form/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormError } from "@shared/ui/form/FormError";
import { validateCuttingDataForm } from "../domain/validateCuttingForm";

import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { Split } from "@shared/ui/primitives/Split/Split";
import { PageShell } from "@shared/ui/page/PageShell";
import { CuttingHistoryPanel } from "./history/CuttingHistoryPanel";
import { useCuttingPageController } from "./useCuttingPageController";
import { Button } from "@shared/ui/primitives/Button/Button";
import "./CuttingDataPage.css";



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
    <>
      {cuttingDataFieldConfig.map((config) => {
        const fieldState = form.fields[config.key];

        return (
          <FormNumberField
            key={config.key}
            label={config.label}
            tooltip={config.tooltip}
            unit={config.unit}
            field={fieldState}
            disabled={fieldState.locked || config.readOnly}
            autoFocus={config.autoFocus}
            onChange={(value) => onFieldChange(config.key, value)}
            ref={navigation.register(config.key)}
            onKeyDown={navigation.handleKeyDown(config.key)}
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
      onReset={onReset}
    >
    {saveButton}
  </FormActions>
  );



 const formContent = (
    <FormLayout
      error={error}
      actions={actions}
      actionsPlacement="bottom"
    >
      {fields}
    </FormLayout>
  );

  /* =========================
     Render
  ========================= */

return (
  <PageShell>
    <Split
      primaryWidth="200px"
      fillHeight
      align="stretch"
      secondaryWidth="minmax(20rem, 1fr)"
      secondaryMinHeightOnCollapse="20rem"
      primary={
        <div ref={navigation.containerRef} className="cutting-data-form-root">
          {formContent}
        </div>
      }
      secondary={
        <CuttingHistoryPanel
          history={history}
          onLoad={load}
          onDelete={remove}
          onClear={clear}
        />
      }
    />
  </PageShell>
);
}


import {
  handleCalculateAsync,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { useFormNavigation } from "@shared/hooks";
import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { Button } from "@shared/ui/primitives/Button/Button";

import { solveCuttingData } from "../../api/solveCuttingData";
import {
  mutuallyExclusiveCuttingDataPairs,
  validCuttingDataInputSets,
} from "../../domain/cuttingDataConstraints";
import type { CuttingDataKey } from "../../domain/cuttingDataForm";
import { parseCuttingData } from "../../domain/parseCuttingData";
import { validateCuttingDataForm } from "../../domain/validateCuttingForm";
import { cuttingDataFieldConfig } from "../cuttingDataFieldConfig";
import type { useCuttingPageController } from "../useCuttingPageController";
import "../CuttingDataPage.css";

type Props = {
  controller: ReturnType<typeof useCuttingPageController>;
};

const focusOrder: CuttingDataKey[] = [
  "diameter",
  "rpm",
  "cutting_speed",
  "teeth",
  "feed_rate",
  "chip_load",
];

export function CuttingDataForm({ controller }: Props) {
  const {
    form,
    setForm,
    save,
    resetForm,
  } = controller;

  const navigation = useFormNavigation({
    keys: cuttingDataFieldConfig.map((fieldConfig) => fieldConfig.key),
    autoFocusOnMount: true,
    activePath: "/cutting",
    onSubmit: onCalculate,
  });

  function onFieldChange(key: CuttingDataKey, value: string) {
    setForm((prev) =>
      handleUserEdit(
        prev,
        key,
        value,
        validCuttingDataInputSets,
        mutuallyExclusiveCuttingDataPairs,
      ),
    );
  }

  async function onCalculate() {
    const next = await handleCalculateAsync(
      form,
      parseCuttingData,
      (input) => solveCuttingData(input),
      validateCuttingDataForm,
    );

    setForm(next);
    const hasInlineError = focusOrder.some((key) =>
      Boolean(next.fields[key].error),
    );

    if (hasInlineError) {
      navigation.focusFirstInvalidAfterRender((key) =>
        Boolean(next.fields[key].error),
      );
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

  const error = form.formError ? (
    <FormError error={form.formError} />
  ) : null;

  return (
    <div ref={navigation.containerRef} className="cutting-data-form-root">
      <FormLayout
        error={error}
        actions={(
          <FormActions
            onCalculate={onCalculate}
            onReset={onReset}
          >
            <Button
              variant="secondary"
              size="medium"
              onClick={save}
            >
              Save result
            </Button>
          </FormActions>
        )}
        actionsPlacement="bottom"
      >
        <FormGrid areas={[["fields"]]}>
          <FormGrid.Area name="fields">
            <FormNumberFields
              configs={cuttingDataFieldConfig}
              fields={form.fields}
              onChange={onFieldChange}
              register={navigation.register}
              onKeyDown={navigation.handleKeyDown}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

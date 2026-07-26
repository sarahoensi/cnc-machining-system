import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { Button } from "@shared/ui/primitives/Button/Button";

import { cuttingDataFieldConfig } from "../cuttingDataFieldConfig";
import type { useCuttingPageController } from "../useCuttingPageController";
import "../CuttingDataPage.css";

type Props = {
  controller: ReturnType<typeof useCuttingPageController>;
};

export function CuttingDataForm({ controller }: Props) {
  const {
    form,
    navigation,
    save,
  } = controller;

  const error = form.formError ? (
    <FormError error={form.formError} />
  ) : null;

  return (
    <div ref={navigation.containerRef} className="cutting-data-form-root">
      <FormLayout
        error={error}
        actions={(
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
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
              onChange={controller.onFieldChange}
              register={navigation.register}
              onKeyDown={navigation.handleKeyDown}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

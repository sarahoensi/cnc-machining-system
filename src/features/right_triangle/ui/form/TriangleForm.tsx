import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";

import { triangleFieldConfig } from "../triangleFieldConfig";
import type { useTrianglePageController } from "../useTrianglePageController";

type Props = {
  controller: ReturnType<typeof useTrianglePageController>;
};

export function TriangleForm({ controller }: Props) {
  const { form, navigation } = controller;

  const error = form.formError ? <FormError error={form.formError} /> : null;

  return (
    <div ref={navigation.containerRef}>
      <FormLayout
        error={error}
        actions={
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
          />
        }
      >
        <FormGrid areas={[["fields"]]}>
          <FormGrid.Area name="fields">
            <FormNumberFields
              configs={triangleFieldConfig}
              fields={form.fields}
              onChange={controller.onFieldChange}
              register={navigation.register}
              onKeyDown={navigation.handleKeyDown}
              onFocusField={navigation.onFieldFocus}
              onBlurFields={navigation.onFieldBlur}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

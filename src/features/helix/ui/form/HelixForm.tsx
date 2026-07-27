import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormModeField } from "@shared/ui/form/fields/FormModeField";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";

import { helixFieldConfig } from "../helixFieldConfig";
import { helixTooltips } from "../helixTooltip";
import type { useHelixPageController } from "../useHelixPageController";

type Props = {
  controller: ReturnType<typeof useHelixPageController>;
};

export function HelixForm({ controller }: Props) {
  const { form, navigation } = controller;

  const error = form.formError ? <FormError error={form.formError} /> : null;

  const fields = (
    <FormGrid areas={[["mode"], ["fields"]]}>
      <FormGrid.Area name="mode" className="stack--form-section">
        <FormModeField
          label="Mode"
          tooltip={helixTooltips.mode}
          value={form.extras.mode}
          onChange={controller.onModeChange}
          options={[
            { value: "Outer", label: "Outer" },
            { value: "Inner", label: "Inner" },
          ]}
        />
      </FormGrid.Area>

      <FormGrid.Area name="fields" className="stack--form-section">
        <FormNumberFields
          configs={helixFieldConfig}
          fields={form.fields}
          onChange={controller.onFieldChange}
          register={navigation.register}
          onKeyDown={navigation.handleKeyDown}
          onFocusField={navigation.onFieldFocus}
          onBlurFields={navigation.onFieldBlur}
        />
      </FormGrid.Area>
    </FormGrid>
  );

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
        {fields}
      </FormLayout>
    </div>
  );
}

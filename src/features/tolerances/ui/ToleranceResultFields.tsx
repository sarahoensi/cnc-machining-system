import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";

import type { ToleranceFormState } from "../domain/toleranceForm";
import { toleranceResultFieldConfig } from "./toleranceFieldConfig";

export function ToleranceResultFields({
  form,
}: {
  form: ToleranceFormState;
}) {
  return (
    <FormSection variant="result">
      {toleranceResultFieldConfig.map((fieldConfig) => (
        <FormNumberField
          key={fieldConfig.key}
          label={fieldConfig.label}
          tooltip={fieldConfig.tooltip}
          unit={fieldConfig.unit}
          field={form.fields[fieldConfig.key]}
          readonly={fieldConfig.readOnly}
          onChange={() => undefined}
        />
      ))}
    </FormSection>
  );
}

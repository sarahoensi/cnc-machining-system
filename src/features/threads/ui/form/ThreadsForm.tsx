import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { FormSelectMenuField } from "@shared/ui/form/fields/FormSelectMenuField";
import { Button } from "@shared/ui/primitives/Button/Button";
import { SelectMenuLabel } from "@shared/ui/primitives/Select";

import type { useThreadsPageController } from "../useThreadsPageController";
import { threadResultFieldConfig, threadSelectConfig } from "../threadFieldConfig";
import "../ThreadsPage.css";

type Props = {
  controller: ReturnType<typeof useThreadsPageController>;
};

export function ThreadsForm({ controller }: Props) {
  const { form, navigation } = controller;

  const error = form.formError ? <FormError error={form.formError} /> : null;
  const selectedPitchOption = controller.pitchOptions.find(
    (option) => option.value === form.fields.pitch.value,
  );
  const selectedSizeOption = controller.sizeOptions.find(
    (option) => option.value === form.fields.size.value,
  );

  return (
    <div ref={navigation.containerRef} className="threads-form-root">
      <FormLayout
        error={error}
        actions={
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
            calculateRef={navigation.registerSubmitAction}
            onCalculateKeyDown={navigation.handleSubmitActionKeyDown}
          >
            <Button
              variant="secondary"
              size="medium"
              onClick={controller.save}
              disabled={form.status !== "solved"}
            >
              Save result
            </Button>
          </FormActions>
        }
        actionsPlacement="bottom"
      >
        <FormGrid areas={[["selection"], ["result"]]}>
          <FormGrid.Area name="selection">
            <div className="threads-selection-grid">
              <FormSelectMenuField
                label={threadSelectConfig.type.label}
                tooltip={threadSelectConfig.type.tooltip}
                valueLabel={
                  controller.typeOptions.find(
                    (option) => option.value === controller.type,
                  )?.label ?? "-"
                }
                options={controller.typeOptions}
                onSelect={controller.onTypeChange}
                disabled={controller.loadingOptions}
                staticWhenSingleOption
              />

              <FormSelectMenuField
                label={threadSelectConfig.size.label}
                tooltip={threadSelectConfig.size.tooltip}
                valueLabel={
                  selectedSizeOption?.label ?? (form.fields.size.value || "-")
                }
                options={controller.sizeOptions}
                onSelect={controller.onSizeChange}
                disabled={controller.loadingOptions}
                staticWhenSingleOption
                ref={navigation.register("size")}
                onKeyDown={navigation.handleKeyDown("size")}
              />

              <FormSelectMenuField
                label={threadSelectConfig.pitch.label}
                tooltip={threadSelectConfig.pitch.tooltip}
                valueLabel={
                  selectedPitchOption ? (
                    <SelectMenuLabel
                      label={selectedPitchOption.label}
                      meta={selectedPitchOption.meta}
                    />
                  ) : (
                    "-"
                  )
                }
                options={controller.pitchOptions}
                onSelect={controller.onPitchChange}
                disabled={controller.loadingOptions}
                staticWhenSingleOption
                ref={navigation.register("pitch")}
                onKeyDown={navigation.handleKeyDown("pitch")}
              />
              {selectedPitchOption ? (
                <div className="threads-pitch-meta">
                  {formatMillimeters(selectedPitchOption.pitchMm)} mm
                </div>
              ) : null}
            </div>
          </FormGrid.Area>

          <FormGrid.Area name="result" className="threads-result-grid">
            <FormNumberFields
              configs={threadResultFieldConfig}
              fields={form.fields}
              onChange={controller.onFieldChange}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

function formatMillimeters(value: number) {
  return value.toFixed(3);
}

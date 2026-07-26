import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";

import type { CylinderWeightKey } from "../../domain/cylinderWeightForm";
import { cylinderWeightFieldConfig } from "../cylinderWeightFieldConfig";
import { MaterialField } from "../materials/field/MaterialField";
import type { useCylinderWeightPageController } from "../useCylinderWeightPageController";

type Props = {
  controller: ReturnType<typeof useCylinderWeightPageController>;
};

const inputFieldConfigs = cylinderWeightFieldConfig.filter(
  (fieldConfig) => !fieldConfig.readOnly,
) as Array<
  (typeof cylinderWeightFieldConfig)[number] & {
    key: Exclude<CylinderWeightKey, "mass_kg">;
  }
>;

const resultFieldConfigs = cylinderWeightFieldConfig.filter(
  (fieldConfig) => fieldConfig.readOnly,
);

export function CylinderWeightForm({ controller }: Props) {
  const { navigation } = controller;

  const error = controller.form.formError ? (
    <FormError error={controller.form.formError} />
  ) : null;

  return (
    <div ref={navigation.containerRef}>
      <FormLayout
        error={error}
        actions={(
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
            disabled={controller.loadingMaterials}
          />
        )}
      >
        <FormGrid areas={[["material"], ["inputs"], ["result"]]}>
          <FormGrid.Area name="material" className="stack--form-section">
            <MaterialField
              materials={controller.materials}
              selectedMaterial={controller.selectedMaterial}
              onMaterialChange={controller.onMaterialChange}
              onOpenManage={() => controller.manageModal.setOpen(true)}
              onOpenCreate={() => controller.manageModal.setNewMaterialOpen(true)}
              materialLoadError={controller.materialLoadError}
            />
          </FormGrid.Area>

          <FormGrid.Area name="inputs" className="stack--form-section">
            <FormNumberFields
              configs={inputFieldConfigs}
              fields={controller.form.fields}
              onChange={controller.onFieldChange}
              register={navigation.register}
              onKeyDown={navigation.handleKeyDown}
            />
          </FormGrid.Area>

          <FormGrid.Area name="result" className="cylinder-weight-result-section">
            <FormNumberFields
              configs={resultFieldConfigs}
              fields={controller.form.fields}
              onChange={controller.onFieldChange}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

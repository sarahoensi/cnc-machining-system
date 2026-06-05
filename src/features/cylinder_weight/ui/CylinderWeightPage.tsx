// src/features/cylinder_weight/ui/CylinderWeightPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormError } from "@shared/ui/form/FormError";
import { FormActions } from "@shared/ui/form/FormActions";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { FormPage } from "@shared/ui/page/FormPage";
import { Stack } from "@shared/ui/primitives/Stack/Stack";
import { useFormNavigation } from "@shared/hooks";
import { cylinderWeightFieldConfig } from "./cylinderWeightFieldConfig";
import { CylinderWeightKey } from "../domain/cylinderWeightForm";
import { useCylinderWeightPageController } from "./useCylinderWeightPageController";
import { MaterialField } from "./materials/field/MaterialField";
import { ManageMaterialsModal } from "./materials/manage/ManageMaterialsModal";
import { NewMaterialModal } from "./materials/create/NewMaterialModal";
import { MaterialResultDialogs } from "./materials/feedback/MaterialResultDialogs";
import "./CylinderWeightPage.css";
import { ExportMaterialsModal } from "./materials";

export function CylinderWeightPage() {
  usePageTitle("Cylinder Weight");
  const controller = useCylinderWeightPageController();

  const navigation = useFormNavigation({
    keys: ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
    autoFocusOnMount: true,
    activePath: "/cylinder-weight",
    onSubmit: onCalculate,
  });

  async function onCalculate() {
    const next = await controller.calculate();
    if (!next) return;
    const focusOrder: Exclude<CylinderWeightKey, "mass_kg">[] = [
      "outer_diameter_mm",
      "inner_diameter_mm",
      "length_mm",
    ];
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
    controller.resetForm();
    navigation.focusFirstAfterRender();
  }

  const fields = (
    <>
      <Stack className="stack--form-section">
        <MaterialField
          materials={controller.materials}
          selectedMaterial={controller.selectedMaterial}
          onMaterialChange={controller.onMaterialChange}
          onOpenManage={() => controller.manageModal.setOpen(true)}
          onOpenCreate={() => controller.manageModal.setNewMaterialOpen(true)}
          materialLoadError={controller.materialLoadError}
        />

        {cylinderWeightFieldConfig
          .filter((f) => !f.readOnly)
          .map((f) => {
            const fieldState = controller.form.fields[f.key];
            return (
              <FormNumberField
                key={f.key}
                label={f.label}
                tooltip={f.tooltip}
                unit={f.unit}
                field={fieldState}
                autoFocus={f.autoFocus}
                disabled={fieldState.locked}
                readonly={f.readOnly}
                onChange={(value) => controller.onFieldChange(f.key, value)}
                ref={navigation.register(f.key as Exclude<CylinderWeightKey, "mass_kg">)}
                onKeyDown={navigation.handleKeyDown(f.key as Exclude<CylinderWeightKey, "mass_kg">)}
              />
            );
          })}
      </Stack>

      <div className="cylinder-weight-result-section">
        {cylinderWeightFieldConfig
          .filter((f) => f.readOnly)
          .map((f) => {
            const fieldState = controller.form.fields[f.key];
            return (
              <FormNumberField
                key={f.key}
                label={f.label}
                tooltip={f.tooltip}
                unit={f.unit}
                field={fieldState}
                autoFocus={f.autoFocus}
                disabled={fieldState.locked}
                readonly={f.readOnly}
                onChange={(value) => controller.onFieldChange(f.key, value)}
              />
            );
          })}
      </div>
    </>
  );

  const error = controller.form.formError ? (
    <FormError error={controller.form.formError} />
  ) : null;

  const actions = (
    <FormActions
      onCalculate={onCalculate}
      onReset={onReset}
      disabled={controller.loadingMaterials}
    />
  );

  const formContent = (
    <FormLayout
      error={error}
      actions={actions}
    >
      {fields}
    </FormLayout>
  );

  return (
    <>
      <div className="cylinder-weight-page-layout">
        <div ref={navigation.containerRef}>
          <FormPage form={formContent} panelWidth="320px" />
        </div>
      </div>

      <ManageMaterialsModal
        open={controller.manageModal.open}
        onClose={() => {
          controller.editMaterial.cancel();
          controller.manageModal.setOpen(false);
        }}
        materials={controller.materials}
        onOpenCreate={() => {
          controller.editMaterial.cancel();
          controller.createMaterial.setError(undefined);
          controller.manageModal.setNewMaterialOpen(true);
        }}
        onOpenExport={controller.importExport.openExportDialog}
        onImportFile={controller.importExport.onImportMaterialsFile}
        edit={{
          id: controller.editMaterial.id,
          name: controller.editMaterial.name,
          setName: controller.editMaterial.setName,
          density: controller.editMaterial.density,
          setDensity: controller.editMaterial.setDensity,
          error: controller.editMaterial.error,
          start: controller.editMaterial.start,
          cancel: controller.editMaterial.cancel,
          save: controller.editMaterial.save,
          remove: controller.editMaterial.remove,
        }}
      />

      <NewMaterialModal
        open={controller.manageModal.newMaterialOpen}
        onClose={() => {
          controller.manageModal.setNewMaterialOpen(false);
          controller.createMaterial.setError(undefined);
        }}
        name={controller.createMaterial.name}
        setName={controller.createMaterial.setName}
        density={controller.createMaterial.density}
        setDensity={controller.createMaterial.setDensity}
        error={controller.createMaterial.error}
        onSave={controller.createMaterial.save}
      />

      <ExportMaterialsModal
        open={controller.manageModal.exportOpen}
        onClose={controller.importExport.cancelExportDialog}
        materials={controller.materials}
        selectedIds={controller.importExport.selectedExportIds}
        onSetAll={controller.importExport.setExportAll}
        onToggle={controller.importExport.toggleExportMaterial}
        onConfirm={controller.importExport.confirmExportSelected}
      />

      <MaterialResultDialogs
        importSummary={controller.importExport.importSummary}
        exportSummary={controller.importExport.exportSummary}
        onCloseImport={() => controller.importExport.setImportSummary(null)}
        onCloseExport={() => controller.importExport.setExportSummary(null)}
      />
    </>
  );
}



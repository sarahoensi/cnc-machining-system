// src/features/cylinder_weight/ui/CylinderWeightPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormPage } from "@shared/ui/page/FormPage";
import { useCylinderWeightPageController } from "./useCylinderWeightPageController";
import { ManageMaterialsModal } from "./materials/manage/ManageMaterialsModal";
import { NewMaterialModal } from "./materials/create/NewMaterialModal";
import { MaterialResultDialogs } from "./materials/feedback/MaterialResultDialogs";
import "./CylinderWeightPage.css";
import { ExportMaterialsModal } from "./materials";
import { CylinderWeightForm } from "./form/CylinderWeightForm";

export function CylinderWeightPage() {
  usePageTitle("Cylinder Weight");
  const controller = useCylinderWeightPageController();

  return (
    <>
      <div className="cylinder-weight-page-layout">
        <FormPage
          form={<CylinderWeightForm controller={controller} />}
          panelWidth="320px"
        />
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



import { useCylinderMaterialManagement } from "./useCylinderMaterialManagement";
import { useCylinderMaterials } from "./useCylinderMaterials";
import { useCylinderWeightCalculation } from "./useCylinderWeightCalculation";

export function useCylinderWeightPageController() {
  const calculation = useCylinderWeightCalculation();
  const materialState = useCylinderMaterials(calculation.form, calculation.setForm);
  const materialManagement = useCylinderMaterialManagement({
    materials: materialState.materials,
    setMaterials: materialState.setMaterials,
    selectedMaterialId: calculation.form.extras.materialId,
    loadMaterials: materialState.loadMaterials,
    selectMaterial: materialState.selectMaterial,
  });

  return {
    form: calculation.form,
    navigation: calculation.navigation,
    onFieldChange: calculation.onFieldChange,
    calculate: calculation.calculate,
    resetForm: calculation.resetForm,

    materials: materialState.materials,
    selectedMaterial: materialState.selectedMaterial,
    loadingMaterials: materialState.loadingMaterials,
    materialLoadError: materialState.materialLoadError,
    onMaterialChange: materialState.onMaterialChange,

    manageModal: materialManagement.manageModal,
    createMaterial: materialManagement.createMaterial,
    editMaterial: materialManagement.editMaterial,
    importExport: materialManagement.importExport,
  };
}

import { useFeatureForm } from "@app/providers/FormStateProvider";
import {
  clearMachineFields,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";
import { getTauriCommandError } from "@shared/api/tauriError";
import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { useEffect, useMemo, useState } from "react";
import {
  createCylinderMaterialApi,
  deleteCylinderMaterialApi,
  importCylinderMaterialsApi,
  listCylinderMaterialsApi,
  updateCylinderMaterialApi,
} from "../api/client";
import { solveCylinderWeight } from "../api/solveCylinderWeight";
import {
  createInitialCylinderWeightForm,
  CylinderWeightKey,
} from "../domain/cylinderWeightForm";
import { parseCylinderWeight } from "../domain/parseCylinderWeight";
import { validateCylinderWeightForm } from "../domain/validateCylinderWeightForm";
import { handleUserEdit } from "@shared/form/engine/formEngine";
import { CylinderMaterial, ExportSummary, ImportSummary } from "./materials";
import { sortCylinderMaterials } from "./materials/sortMaterials";

const validInputSets = [
  ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
] as const;

const mutuallyExclusivePairs = [] as const;

export function useCylinderWeightPageController() {
  const [form, setForm] = useFeatureForm(
    "cylinder_weight",
    createInitialCylinderWeightForm
  );

  const [materials, setMaterials] = useState<CylinderMaterial[]>([]);
  const [loadingMaterials, setLoadingMaterials] = useState(true);
  const [materialLoadError, setMaterialLoadError] = useState<string>();

  const [newMaterialName, setNewMaterialName] = useState("");
  const [newMaterialDensity, setNewMaterialDensity] = useState("");
  const [createMaterialError, setCreateMaterialError] = useState<string>();

  const [editMaterialId, setEditMaterialId] = useState("");
  const [editMaterialName, setEditMaterialName] = useState("");
  const [editMaterialDensity, setEditMaterialDensity] = useState("");
  const [editMaterialError, setEditMaterialError] = useState<string>();

  const [importSummary, setImportSummary] = useState<ImportSummary | null>(null);
  const [exportSummary, setExportSummary] = useState<ExportSummary | null>(null);

  const [isManageOpen, setIsManageOpen] = useState(false);
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [isExportOpen, setIsExportOpen] = useState(false);
  const [selectedExportIds, setSelectedExportIds] = useState<string[]>([]);

  useEffect(() => {
    void loadMaterials();
  }, []);

  async function loadMaterials() {
    setLoadingMaterials(true);
    setMaterialLoadError(undefined);
    try {
      const rows = await listCylinderMaterialsApi();
      setMaterials(sortCylinderMaterials(rows));

      if (!form.extras.materialId && rows.length > 0) {
        setForm((prev) => ({
          ...prev,
          extras: {
            ...prev.extras,
            materialId: rows[0].id,
            materialName: rows[0].name,
            densityKgM3: rows[0].density_kg_m3,
          },
        }));
      }
    } catch (error) {
      if (error instanceof Error) setMaterialLoadError(error.message);
      else setMaterialLoadError("Failed to load materials");
    } finally {
      setLoadingMaterials(false);
    }
  }

  function onFieldChange(key: CylinderWeightKey, value: string) {
    setForm((prev) =>
      handleUserEdit(prev, key, value, validInputSets, mutuallyExclusivePairs)
    );
  }

  function onMaterialChange(materialId: string) {
    const selected = materials.find((m) => m.id === materialId);
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: clearMachineFields(prev.fields),
      extras: {
        ...prev.extras,
        materialId,
        materialName: selected?.name,
        densityKgM3: selected?.density_kg_m3,
      },
      formError: undefined,
    }));
  }

  async function onCreateMaterial() {
    setCreateMaterialError(undefined);
    const density = safeParseDecimal(newMaterialDensity);
    if (!newMaterialName.trim()) return setCreateMaterialError("Material name is required");
    if (density == null) return setCreateMaterialError("Density must be a valid number");

    try {
      const saved = await createCylinderMaterialApi({
        name: newMaterialName,
        density_kg_m3: density,
      });
      setMaterials((prev) => sortCylinderMaterials([...prev, saved]));
      setNewMaterialName("");
      setNewMaterialDensity("");
      onMaterialChange(saved.id);
      setIsCreateDialogOpen(false);
    } catch (error) {
      const te = getTauriCommandError(error);
      if (te) setCreateMaterialError(te.message);
      else if (error instanceof Error) setCreateMaterialError(error.message);
      else setCreateMaterialError("Failed to create material");
    }
  }

  function startEditMaterial(material: CylinderMaterial) {
    setEditMaterialError(undefined);
    setEditMaterialId(material.id);
    setEditMaterialName(material.name);
    setEditMaterialDensity(String(material.density_kg_m3));
  }

  function cancelEditMaterial() {
    setEditMaterialId("");
    setEditMaterialName("");
    setEditMaterialDensity("");
    setEditMaterialError(undefined);
  }

  async function onSaveEditMaterial() {
    setEditMaterialError(undefined);
    const density = safeParseDecimal(editMaterialDensity);
    if (!editMaterialId) return setEditMaterialError("Material id is missing");
    if (!editMaterialName.trim()) return setEditMaterialError("Material name is required");
    if (density == null) return setEditMaterialError("Density must be a valid number");
    try {
      const updated = await updateCylinderMaterialApi({
        id: editMaterialId,
        name: editMaterialName,
        density_kg_m3: density,
      });
      setMaterials((prev) =>
        sortCylinderMaterials(prev.map((m) => (m.id === updated.id ? updated : m)))
      );
      if (form.extras.materialId === updated.id) onMaterialChange(updated.id);
      cancelEditMaterial();
    } catch (error) {
      const te = getTauriCommandError(error);
      if (te) setEditMaterialError(te.message);
      else if (error instanceof Error) setEditMaterialError(error.message);
      else setEditMaterialError("Failed to update material");
    }
  }

  async function onDeleteMaterial(materialId: string) {
    try {
      await deleteCylinderMaterialApi({ id: materialId });
      const next = materials.filter((m) => m.id !== materialId);
      setMaterials(sortCylinderMaterials(next));
      if (editMaterialId === materialId) cancelEditMaterial();
      if (form.extras.materialId === materialId) onMaterialChange(next[0]?.id ?? "");
    } catch (error) {
      const te = getTauriCommandError(error);
      if (te) setEditMaterialError(te.message);
      else if (error instanceof Error) setEditMaterialError(error.message);
      else setEditMaterialError("Failed to delete material");
    }
  }

  async function onImportMaterialsFile(file: File) {
    setImportSummary(null);
    try {
      const jsonPayload = await file.text();
      const result = await importCylinderMaterialsApi({ json_payload: jsonPayload });
      await loadMaterials();
      setImportSummary({
        imported: result.imported,
        skippedDuplicates: result.skipped_duplicates,
        skippedInvalid: result.skipped_invalid,
        added: result.added,
        skipped: result.skipped,
      });
    } catch (error) {
      const te = getTauriCommandError(error);
      if (te) setEditMaterialError(te.message);
      else if (error instanceof Error) setEditMaterialError(error.message);
      else setEditMaterialError("Import failed");
    }
  }

  function openExportDialog() {
    setSelectedExportIds(materials.map((m) => m.id));
    setIsExportOpen(true);
  }

  function toggleExportMaterial(id: string) {
    setSelectedExportIds((prev) =>
      prev.includes(id) ? prev.filter((x) => x !== id) : [...prev, id]
    );
  }

  function setExportAll(checked: boolean) {
    setSelectedExportIds(checked ? materials.map((m) => m.id) : []);
  }

  function cancelExportDialog() {
    setIsExportOpen(false);
    setSelectedExportIds([]);
  }

  function confirmExportSelected() {
    setExportSummary(null);
    const selectedRows = materials.filter((m) => selectedExportIds.includes(m.id));
    if (selectedRows.length === 0) return;

    const payload = {
      schema_version: 1,
      materials: selectedRows.map((m) => ({
        name: m.name,
        density_kg_m3: m.density_kg_m3,
      })),
    };
    const json = JSON.stringify(payload, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "cylinder_materials.json";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    setIsExportOpen(false);
    setSelectedExportIds([]);
    setExportSummary({
      exported: selectedRows.length,
      materials: selectedRows.map((row) => ({
        name: row.name,
        density_kg_m3: row.density_kg_m3,
      })),
    });
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseCylinderWeight,
      solveCylinderWeight,
      validateCylinderWeightForm
    );
    setForm(next);
  }

  function resetForm() {
    setForm((prev) => {
      const initial = createInitialCylinderWeightForm();
      return {
        ...initial,
        extras: {
          ...initial.extras,
          materialId: prev.extras.materialId,
          materialName: prev.extras.materialName,
          densityKgM3: prev.extras.densityKgM3,
        },
      };
    });
  }

  const selectedMaterial = useMemo(
    () => materials.find((m) => m.id === form.extras.materialId),
    [materials, form.extras.materialId]
  );

  return {
    form,
    onFieldChange,
    calculate,
    resetForm,

    materials,
    selectedMaterial,
    loadingMaterials,
    materialLoadError,
    onMaterialChange,

    manageModal: {
      open: isManageOpen,
      setOpen: setIsManageOpen,
      newMaterialOpen: isCreateDialogOpen,
      setNewMaterialOpen: setIsCreateDialogOpen,
      exportOpen: isExportOpen,
      setExportOpen: setIsExportOpen,
    },

    createMaterial: {
      name: newMaterialName,
      setName: setNewMaterialName,
      density: newMaterialDensity,
      setDensity: setNewMaterialDensity,
      error: createMaterialError,
      setError: setCreateMaterialError,
      save: onCreateMaterial,
    },

    editMaterial: {
      id: editMaterialId,
      name: editMaterialName,
      setName: setEditMaterialName,
      density: editMaterialDensity,
      setDensity: setEditMaterialDensity,
      error: editMaterialError,
      start: startEditMaterial,
      cancel: cancelEditMaterial,
      save: onSaveEditMaterial,
      remove: onDeleteMaterial,
    },

    importExport: {
      importSummary,
      setImportSummary,
      exportSummary,
      setExportSummary,
      onImportMaterialsFile,
      openExportDialog,
      selectedExportIds,
      toggleExportMaterial,
      setExportAll,
      cancelExportDialog,
      confirmExportSelected,
    },
  };
}

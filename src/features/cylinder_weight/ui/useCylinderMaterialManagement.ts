import { useState, type Dispatch, type SetStateAction } from "react";

import { getTauriCommandError } from "@shared/api/tauriError";
import { safeParseDecimal } from "@shared/parsing/decimalParser";

import {
  createCylinderMaterialApi,
  deleteCylinderMaterialApi,
  importCylinderMaterialsApi,
  updateCylinderMaterialApi,
} from "../api/client";
import type { CylinderMaterial, ExportSummary, ImportSummary } from "./materials";
import { sortCylinderMaterials } from "./materials/sortMaterials";

type UseCylinderMaterialManagementOptions = {
  materials: CylinderMaterial[];
  setMaterials: Dispatch<SetStateAction<CylinderMaterial[]>>;
  selectedMaterialId: string;
  loadMaterials: () => Promise<void>;
  selectMaterial: (materialId: string, knownMaterial?: CylinderMaterial) => void;
};

export function useCylinderMaterialManagement({
  materials,
  setMaterials,
  selectedMaterialId,
  loadMaterials,
  selectMaterial,
}: UseCylinderMaterialManagementOptions) {
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

  async function onCreateMaterial() {
    setCreateMaterialError(undefined);
    const density = safeParseDecimal(newMaterialDensity);
    if (!newMaterialName.trim())
      return setCreateMaterialError("Material name is required");
    if (density == null)
      return setCreateMaterialError("Density must be a valid number");

    try {
      const saved = await createCylinderMaterialApi({
        name: newMaterialName,
        density_kg_m3: density,
      });
      setMaterials((prev) => sortCylinderMaterials([...prev, saved]));
      setNewMaterialName("");
      setNewMaterialDensity("");
      selectMaterial(saved.id, saved);
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
    if (!editMaterialName.trim())
      return setEditMaterialError("Material name is required");
    if (density == null) return setEditMaterialError("Density must be a valid number");

    try {
      const updated = await updateCylinderMaterialApi({
        id: editMaterialId,
        name: editMaterialName,
        density_kg_m3: density,
      });
      setMaterials((prev) =>
        sortCylinderMaterials(prev.map((m) => (m.id === updated.id ? updated : m))),
      );
      if (selectedMaterialId === updated.id) selectMaterial(updated.id, updated);
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
      const next = sortCylinderMaterials(materials.filter((m) => m.id !== materialId));
      setMaterials(next);
      if (editMaterialId === materialId) cancelEditMaterial();
      if (selectedMaterialId === materialId) selectMaterial(next[0]?.id ?? "", next[0]);
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
      prev.includes(id) ? prev.filter((x) => x !== id) : [...prev, id],
    );
  }

  function setExportAll(checked: boolean, visibleIds: string[]) {
    setSelectedExportIds((prev) => {
      if (checked) {
        return Array.from(new Set([...prev, ...visibleIds]));
      }

      return prev.filter((id) => !visibleIds.includes(id));
    });
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

  return {
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

import { useCallback, useEffect, useMemo, useState } from "react";

import { clearMachineFields } from "@shared/form/engine/formEngine";

import type { createInitialCylinderWeightForm } from "../domain/cylinderWeightForm";
import { listCylinderMaterialsApi } from "../api/client";
import { sortCylinderMaterials } from "./materials/sortMaterials";
import type { CylinderMaterial } from "./materials";

type CylinderWeightFormState = ReturnType<typeof createInitialCylinderWeightForm>;

type SetCylinderWeightForm = (
  value:
    | CylinderWeightFormState
    | ((prev: CylinderWeightFormState) => CylinderWeightFormState),
) => void;

export function useCylinderMaterials(
  form: CylinderWeightFormState,
  setForm: SetCylinderWeightForm,
) {
  const [materials, setMaterials] = useState<CylinderMaterial[]>([]);
  const [loadingMaterials, setLoadingMaterials] = useState(true);
  const [materialLoadError, setMaterialLoadError] = useState<string>();

  const loadMaterials = useCallback(async () => {
    setLoadingMaterials(true);
    setMaterialLoadError(undefined);
    try {
      const rows = await listCylinderMaterialsApi();
      const sortedRows = sortCylinderMaterials(rows);
      setMaterials(sortedRows);

      if (sortedRows.length > 0) {
        setForm((prev) => applyDefaultMaterialSelection(prev, sortedRows));
      }
    } catch (error) {
      if (error instanceof Error) setMaterialLoadError(error.message);
      else setMaterialLoadError("Failed to load materials");
    } finally {
      setLoadingMaterials(false);
    }
  }, [setForm]);

  useEffect(() => {
    void loadMaterials();
  }, [loadMaterials]);

  function selectMaterial(materialId: string, knownMaterial?: CylinderMaterial) {
    setForm((prev) =>
      applyMaterialSelection(prev, materialId, materials, knownMaterial),
    );
  }

  function onMaterialChange(materialId: string) {
    selectMaterial(materialId);
  }

  function upsertMaterial(material: CylinderMaterial) {
    setMaterials((prev) =>
      sortCylinderMaterials([
        ...prev.filter((row) => row.id !== material.id),
        material,
      ]),
    );
  }

  function removeMaterial(materialId: string) {
    setMaterials((prev) => sortCylinderMaterials(prev.filter((m) => m.id !== materialId)));
  }

  const selectedMaterial = useMemo(
    () => materials.find((m) => m.id === form.extras.materialId),
    [materials, form.extras.materialId],
  );

  return {
    materials,
    setMaterials,
    loadingMaterials,
    materialLoadError,
    loadMaterials,
    onMaterialChange,
    selectMaterial,
    upsertMaterial,
    removeMaterial,
    selectedMaterial,
  };
}

export function applyDefaultMaterialSelection(
  form: CylinderWeightFormState,
  materials: CylinderMaterial[],
): CylinderWeightFormState {
  const first = materials[0];
  if (!first) return form;

  return {
    ...form,
    extras: {
      ...form.extras,
      materialId: form.extras.materialId || first.id,
      materialName: form.extras.materialName || first.name,
      densityKgM3: form.extras.densityKgM3 ?? first.density_kg_m3,
    },
  };
}

export function applyMaterialSelection(
  form: CylinderWeightFormState,
  materialId: string,
  materials: CylinderMaterial[],
  knownMaterial?: CylinderMaterial,
): CylinderWeightFormState {
  const selected = knownMaterial ?? materials.find((m) => m.id === materialId);

  return {
    ...form,
    status: "editing",
    fields: clearMachineFields(form.fields),
    extras: {
      ...form.extras,
      materialId,
      materialName: selected?.name,
      densityKgM3: selected?.density_kg_m3,
    },
    formError: undefined,
  };
}

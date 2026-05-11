import { useEffect, useMemo, useState } from "react";
import { useFeatureForm } from "@app/providers/FormStateProvider";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import {
  clearMachineFields,
  handleCalculateAsync,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { Field } from "@shared/ui/components/form/Field/Field";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { Panel } from "@shared/ui/layout/container/Panel/Panel";
import { FormFigureLayout } from "@shared/ui/layout/page/FormFigureLayout/FormFigureLayout";
import { NumberInput } from "@shared/ui/primitives/NumberInput/NumberInput";
import { Button } from "@shared/ui/primitives/Button/Button";
import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  createCylinderMaterialApi,
  deleteCylinderMaterialApi,
  listCylinderMaterialsApi,
  updateCylinderMaterialApi,
} from "../api/client";
import { solveCylinderWeight } from "../api/solveCylinderWeight";
import { CylinderMaterialResponse } from "../api/types";
import {
  createInitialCylinderWeightForm,
  CylinderWeightKey,
} from "../domain/cylinderWeightForm";
import { parseCylinderWeight } from "../domain/parseCylinderWeight";
import { validateCylinderWeightForm } from "../domain/validateCylinderWeightForm";
import { cylinderWeightFieldConfig } from "./cylinderWeightFieldConfig";
import { useFormNavigation } from "@shared/ui";
import "./CylinderWeightPage.css";

const validInputSets = [
  ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
] as const;

const mutuallyExclusivePairs = [] as const;

export function CylinderWeightPage() {
  usePageTitle("Cylinder Weight");

  const [form, setForm] = useFeatureForm(
    "cylinder_weight",
    createInitialCylinderWeightForm
  );

  const [materials, setMaterials] = useState<CylinderMaterialResponse[]>(
    []
  );
  const [loadingMaterials, setLoadingMaterials] = useState(true);
  const [materialLoadError, setMaterialLoadError] = useState<string>();

  const [newMaterialName, setNewMaterialName] = useState("");
  const [newMaterialDensity, setNewMaterialDensity] = useState("");
  const [createMaterialError, setCreateMaterialError] = useState<string>();
  const [editMaterialId, setEditMaterialId] = useState<string>("");
  const [editMaterialName, setEditMaterialName] = useState("");
  const [editMaterialDensity, setEditMaterialDensity] = useState("");
  const [editMaterialError, setEditMaterialError] = useState<string>();

  const navigation = useFormNavigation({
    keys: ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
    autoFocusOnMount: true,
    onSubmit: onCalculate,
  });

  useEffect(() => {
    void loadMaterials();
  }, []);

  async function loadMaterials() {
    setLoadingMaterials(true);
    setMaterialLoadError(undefined);

    try {
      const rows = await listCylinderMaterialsApi();
      setMaterials(rows);

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
      if (error instanceof Error) {
        setMaterialLoadError(error.message);
      } else {
        setMaterialLoadError("Failed to load materials");
      }
    } finally {
      setLoadingMaterials(false);
    }
  }

  function onFieldChange(
    key: CylinderWeightKey,
    value: string
  ) {
    setForm((prev) =>
      handleUserEdit(
        prev,
        key,
        value,
        validInputSets,
        mutuallyExclusivePairs
      )
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
    if (!newMaterialName.trim()) {
      setCreateMaterialError("Material name is required");
      return;
    }
    if (density == null) {
      setCreateMaterialError("Density must be a valid number");
      return;
    }

    try {
      const saved = await createCylinderMaterialApi({
        name: newMaterialName,
        density_kg_m3: density,
      });

      setMaterials((prev) => [...prev, saved]);
      setNewMaterialName("");
      setNewMaterialDensity("");
      onMaterialChange(saved.id);
    } catch (error) {
      const tauriError = getTauriCommandError(error);
      if (tauriError) {
        setCreateMaterialError(tauriError.message);
        return;
      }
      if (error instanceof Error) {
        setCreateMaterialError(error.message);
      } else {
        setCreateMaterialError("Failed to create material");
      }
    }
  }

  function startEditMaterial(material: CylinderMaterialResponse) {
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

    if (!editMaterialId) {
      setEditMaterialError("Material id is missing");
      return;
    }
    if (!editMaterialName.trim()) {
      setEditMaterialError("Material name is required");
      return;
    }
    if (density == null) {
      setEditMaterialError("Density must be a valid number");
      return;
    }

    try {
      const updated = await updateCylinderMaterialApi({
        id: editMaterialId,
        name: editMaterialName,
        density_kg_m3: density,
      });

      setMaterials((prev) =>
        prev.map((m) => (m.id === updated.id ? updated : m))
      );

      if (form.extras.materialId === updated.id) {
        onMaterialChange(updated.id);
      }
      cancelEditMaterial();
    } catch (error) {
      const tauriError = getTauriCommandError(error);
      if (tauriError) {
        setEditMaterialError(tauriError.message);
        return;
      }
      if (error instanceof Error) {
        setEditMaterialError(error.message);
      } else {
        setEditMaterialError("Failed to update material");
      }
    }
  }

  async function onDeleteMaterial(materialId: string) {
    try {
      await deleteCylinderMaterialApi({ id: materialId });
      const next = materials.filter((m) => m.id !== materialId);
      setMaterials(next);
      if (editMaterialId === materialId) {
        cancelEditMaterial();
      }
      if (form.extras.materialId === materialId) {
        const fallback = next[0];
        onMaterialChange(fallback?.id ?? "");
      }
    } catch (error) {
      const tauriError = getTauriCommandError(error);
      if (tauriError) {
        setEditMaterialError(tauriError.message);
        return;
      }
      if (error instanceof Error) {
        setEditMaterialError(error.message);
      } else {
        setEditMaterialError("Failed to delete material");
      }
    }
  }

  async function onCalculate() {
    const next = await handleCalculateAsync(
      form,
      parseCylinderWeight,
      solveCylinderWeight,
      validateCylinderWeightForm
    );

    setForm(next);
  }

  function onReset() {
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

  const fields = (
    <>
      {cylinderWeightFieldConfig.map((f) => {
        const fieldState = form.fields[f.key];

        return (
          <FormNumberField
            key={f.key}
            label={f.label}
            unit={f.unit}
            field={fieldState}
            autoFocus={f.autoFocus}
            disabled={fieldState.locked || f.readOnly}
            onChange={(value) => onFieldChange(f.key, value)}
            ref={
              f.readOnly
                ? undefined
                : navigation.register(f.key as Exclude<CylinderWeightKey, "mass_kg">)
            }
            onKeyDown={
              f.readOnly
                ? undefined
                : navigation.handleKeyDown(f.key as Exclude<CylinderWeightKey, "mass_kg">)
            }
          />
        );
      })}
    </>
  );

  const materialField = (
    <Field label="Material">
      <select
        className="cylinder-weight-material-select"
        value={form.extras.materialId}
        onChange={(e) => onMaterialChange(e.target.value)}
      >
        <option value="">Select material</option>
        {materials.map((m) => (
          <option key={m.id} value={m.id}>
            {m.name}
          </option>
        ))}
      </select>
    </Field>
  );

  const error = form.formError ? (
    <FormError error={form.formError} />
  ) : null;

  const actions = (
    <FormActions
      onCalculate={onCalculate}
      onReset={onReset}
      disabled={loadingMaterials}
    />
  );

  const formContent = (
    <FormLayout
      fields={
        <>
          {materialField}
          {fields}
        </>
      }
      error={error}
      actions={actions}
    />
  );

  const figure = (
    <div className="cylinder-weight-figure">
      <Panel title="Material Info">
        {loadingMaterials ? (
          <div>Loading materials...</div>
        ) : materialLoadError ? (
          <div>{materialLoadError}</div>
        ) : selectedMaterial ? (
          <div>
            {selectedMaterial.name} - {selectedMaterial.density_kg_m3} kg/m3
          </div>
        ) : (
          <div>No material selected</div>
        )}
      </Panel>

      <Panel title="Saved Materials">
        <div className="cylinder-weight-material-list">
          {materials.map((material) => {
            const isEditing = editMaterialId === material.id;

            if (isEditing) {
              return (
                <div
                  key={material.id}
                  className="cylinder-weight-material-row"
                >
                  <div className="cylinder-weight-add-material">
                    <input
                      className="cylinder-weight-input"
                      type="text"
                      value={editMaterialName}
                      onChange={(e) => setEditMaterialName(e.target.value)}
                    />
                    <NumberInput
                      value={editMaterialDensity}
                      onChange={setEditMaterialDensity}
                      unit="kg/m3"
                      className="ni-form ni-user"
                    />
                  </div>
                  <div className="cylinder-weight-material-actions">
                    <Button variant="secondary" size="small" onClick={onSaveEditMaterial}>
                      Save
                    </Button>
                    <Button variant="secondary" size="small" onClick={cancelEditMaterial}>
                      Cancel
                    </Button>
                  </div>
                </div>
              );
            }

            return (
              <div
                key={material.id}
                className="cylinder-weight-material-row"
              >
                <div className="cylinder-weight-material-meta">
                  <strong>{material.name}</strong>
                  <span>{material.density_kg_m3} kg/m3</span>
                </div>
                <div className="cylinder-weight-material-actions">
                  <Button
                    variant="secondary"
                    size="small"
                    onClick={() => startEditMaterial(material)}
                  >
                    Edit
                  </Button>
                  <Button
                    variant="danger"
                    size="small"
                    onClick={() => void onDeleteMaterial(material.id)}
                  >
                    Delete
                  </Button>
                </div>
              </div>
            );
          })}

          {editMaterialError ? (
            <div className="cylinder-weight-material-hint">
              {editMaterialError}
            </div>
          ) : null}
        </div>
      </Panel>

      <Panel title="Add Material">
        <div className="cylinder-weight-add-material">
          <Field label="Name">
            <input
              className="cylinder-weight-input"
              type="text"
              value={newMaterialName}
              onChange={(e) => setNewMaterialName(e.target.value)}
              placeholder="Ex: Bronze"
            />
          </Field>

          <Field label="Density">
            <NumberInput
              value={newMaterialDensity}
              onChange={setNewMaterialDensity}
              unit="kg/m3"
              className="ni-form ni-user"
              placeholder="Ex: 8800"
            />
          </Field>

          <Button
            variant="secondary"
            size="medium"
            onClick={onCreateMaterial}
          >
            Add material
          </Button>

          {createMaterialError ? (
            <div className="cylinder-weight-material-hint">
              {createMaterialError}
            </div>
          ) : null}
        </div>
      </Panel>
    </div>
  );

  return (
    <FormFigureLayout
      form={formContent}
      figure={figure}
    />
  );
}

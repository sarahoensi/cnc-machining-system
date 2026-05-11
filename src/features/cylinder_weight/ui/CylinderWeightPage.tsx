import { useEffect, useMemo, useRef, useState } from "react";
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
import { FormFigureLayout } from "@shared/ui/layout/page/FormFigureLayout/FormFigureLayout";
import { Table } from "@shared/ui/components/table/Table";
import { NumberInput } from "@shared/ui/primitives/NumberInput/NumberInput";
import { Button } from "@shared/ui/primitives/Button/Button";
import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  createCylinderMaterialApi,
  deleteCylinderMaterialApi,
  importCylinderMaterialsApi,
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
  const [importSummary, setImportSummary] = useState<{
    imported: number;
    skippedDuplicates: number;
    skippedInvalid: number;
  } | null>(null);
  const [exportSummary, setExportSummary] = useState<{
    exported: number;
  } | null>(null);
  const [isManageOpen, setIsManageOpen] = useState(false);
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [isExportOpen, setIsExportOpen] = useState(false);
  const [selectedExportIds, setSelectedExportIds] = useState<string[]>([]);
  const [isMaterialMenuOpen, setIsMaterialMenuOpen] = useState(false);
  const materialMenuRef = useRef<HTMLDivElement>(null);
  const importInputRef = useRef<HTMLInputElement>(null);
  const selectAllRef = useRef<HTMLInputElement>(null);

  const navigation = useFormNavigation({
    keys: ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
    autoFocusOnMount: true,
    onSubmit: onCalculate,
  });

  useEffect(() => {
    void loadMaterials();
  }, []);

  useEffect(() => {
    function handleOutsideClick(event: MouseEvent) {
      if (
        materialMenuRef.current &&
        !materialMenuRef.current.contains(event.target as Node)
      ) {
        setIsMaterialMenuOpen(false);
      }
    }

    document.addEventListener("mousedown", handleOutsideClick);
    return () => document.removeEventListener("mousedown", handleOutsideClick);
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
      setIsCreateDialogOpen(false);
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

  async function onImportMaterialsFile(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    const file = event.target.files?.[0];
    if (!file) return;

    setImportSummary(null);
    try {
      const jsonPayload = await file.text();
      const result = await importCylinderMaterialsApi({ json_payload: jsonPayload });

      await loadMaterials();
      setImportSummary({
        imported: result.imported,
        skippedDuplicates: result.skipped_duplicates,
        skippedInvalid: result.skipped_invalid,
      });
    } catch (error) {
      const te = getTauriCommandError(error);
      if (te) {
        setEditMaterialError(te.message);
      } else if (error instanceof Error) {
        setEditMaterialError(error.message);
      } else {
        setEditMaterialError("Import failed");
      }
    } finally {
      if (importInputRef.current) {
        importInputRef.current.value = "";
      }
    }
  }

  function onExportMaterials() {
    setSelectedExportIds(materials.map((m) => m.id));
    setIsExportOpen(true);
  }

  function toggleExportMaterial(id: string) {
    setSelectedExportIds((prev) =>
      prev.includes(id)
        ? prev.filter((x) => x !== id)
        : [...prev, id]
    );
  }

  function setExportAll(checked: boolean) {
    setSelectedExportIds(checked ? materials.map((m) => m.id) : []);
  }

  function onCancelExportDialog() {
    setIsExportOpen(false);
    setSelectedExportIds([]);
  }

  function onConfirmExportSelected() {
    setExportSummary(null);

    const selectedRows = materials.filter((m) =>
      selectedExportIds.includes(m.id)
    );

    if (selectedRows.length === 0) {
      return;
    }

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
    setExportSummary({ exported: selectedRows.length });
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
    <div className="cylinder-weight-material-block">
      <Field label="Material">
        <div className="cylinder-weight-material-control-row">
          <div className="cylinder-weight-material-dropdown" ref={materialMenuRef}>
            <button
              type="button"
              className="cylinder-weight-material-select"
              onClick={() => setIsMaterialMenuOpen((v) => !v)}
            >
              <span className="cylinder-weight-material-select-label">
                {selectedMaterial?.name ?? "Select material"}
              </span>
              <span className="cylinder-weight-material-select-caret" />
            </button>

            {isMaterialMenuOpen ? (
              <div className="cylinder-weight-material-dropdown-menu">
                {materials.map((m) => (
                  <button
                    key={m.id}
                    type="button"
                    className="cylinder-weight-material-dropdown-item"
                    onClick={() => {
                      onMaterialChange(m.id);
                      setIsMaterialMenuOpen(false);
                    }}
                  >
                    <span className="cylinder-weight-material-dropdown-name">
                      {m.name}
                    </span>
                    <span className="cylinder-weight-material-dropdown-density">
                      {"\u00b7"} {m.density_kg_m3} kg/m3
                    </span>
                  </button>
                ))}

                <div className="cylinder-weight-material-dropdown-divider" />

                <button
                  type="button"
                  className="cylinder-weight-material-dropdown-utility"
                  onClick={() => {
                    setIsMaterialMenuOpen(false);
                    setIsManageOpen(true);
                    setIsCreateDialogOpen(true);
                  }}
                >
                  + New Material...
                </button>

                <button
                  type="button"
                  className="cylinder-weight-material-dropdown-utility"
                  onClick={() => {
                    setIsMaterialMenuOpen(false);
                    setIsManageOpen(true);
                  }}
                >
                  Manage Materials...
                </button>
              </div>
            ) : null}
          </div>
        </div>
      </Field>

      <button
        type="button"
        className="cylinder-weight-manage-link"
        onClick={() => setIsManageOpen(true)}
      >
        Manage Materials
      </button>

      {materialLoadError ? (
        <div className="cylinder-weight-material-hint">
          {materialLoadError}
        </div>
      ) : null}
    </div>
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

  const allExportSelected =
    materials.length > 0 &&
    selectedExportIds.length === materials.length;
  const someExportSelected =
    selectedExportIds.length > 0 &&
    selectedExportIds.length < materials.length;

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someExportSelected;
    }
  }, [someExportSelected]);

  return (
    <>
      <FormFigureLayout
        form={formContent}
        figure={null}
      />

      {isManageOpen ? (
        <div
          className="cylinder-weight-modal-backdrop"
          onClick={() => setIsManageOpen(false)}
        >
          <div
            className="cylinder-weight-modal"
            role="dialog"
            aria-modal="true"
            aria-label="Manage materials"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="cylinder-weight-modal-header">
              <h3>Manage Materials</h3>
              <Button variant="secondary" size="small" onClick={() => setIsManageOpen(false)}>
                Close
              </Button>
            </div>

            <div className="cylinder-weight-library-toolbar">
              <Button
                variant="secondary"
                size="small"
                onClick={() => {
                  setCreateMaterialError(undefined);
                  setIsCreateDialogOpen(true);
                }}
              >
                + New Material
              </Button>
              <Button
                variant="secondary"
                size="small"
                onClick={() => importInputRef.current?.click()}
              >
                Import Materials
              </Button>
              <Button
                variant="secondary"
                size="small"
                onClick={onExportMaterials}
              >
                Export Materials
              </Button>
              <input
                ref={importInputRef}
                type="file"
                accept="application/json,.json"
                style={{ display: "none" }}
                onChange={onImportMaterialsFile}
              />
            </div>

            <Table.Root className="cylinder-materials-table">
              <Table.Head>
                <Table.HeadRow>
                  <Table.HeaderCell>Material</Table.HeaderCell>
                  <Table.HeaderCell align="right">Density</Table.HeaderCell>
                  <Table.HeaderCell align="right">Actions</Table.HeaderCell>
                </Table.HeadRow>
              </Table.Head>

              <Table.Body>
                {materials.map((material) => {
                  const isEditing = editMaterialId === material.id;

                  if (isEditing) {
                    return (
                      <Table.BodyRow key={material.id}>
                        <Table.Cell>
                          <input
                            className="cylinder-weight-input cylinder-weight-input-compact"
                            type="text"
                            value={editMaterialName}
                            onChange={(e) => setEditMaterialName(e.target.value)}
                          />
                        </Table.Cell>
                        <Table.Cell align="right">
                          <NumberInput
                            value={editMaterialDensity}
                            onChange={setEditMaterialDensity}
                            unit="kg/m3"
                            className="ni-form ni-user"
                          />
                        </Table.Cell>
                        <Table.Cell align="right">
                          <div className="cylinder-weight-material-actions">
                            <Button variant="link" onClick={onSaveEditMaterial}>
                              Save
                            </Button>
                            <Button variant="link" onClick={cancelEditMaterial}>
                              Cancel
                            </Button>
                          </div>
                        </Table.Cell>
                      </Table.BodyRow>
                    );
                  }

                  return (
                    <Table.BodyRow key={material.id}>
                      <Table.Cell>
                        <span className="cylinder-weight-material-name">
                          {material.name}
                        </span>
                      </Table.Cell>
                      <Table.Cell align="right">
                        <span className="cylinder-weight-material-density">
                          {material.density_kg_m3} kg/m3
                        </span>
                      </Table.Cell>
                      <Table.Cell align="right">
                        <div className="cylinder-weight-material-actions">
                          <Button variant="link" onClick={() => startEditMaterial(material)}>
                            Edit
                          </Button>
                          <Button
                            variant="link"
                            className="cylinder-weight-action-delete"
                            onClick={() => void onDeleteMaterial(material.id)}
                          >
                            Delete
                          </Button>
                        </div>
                      </Table.Cell>
                    </Table.BodyRow>
                  );
                })}
              </Table.Body>
            </Table.Root>

            {editMaterialError ? (
              <div className="cylinder-weight-material-hint">
                {editMaterialError}
              </div>
            ) : null}

          </div>
        </div>
      ) : null}

      {isCreateDialogOpen ? (
        <div
          className="cylinder-weight-modal-backdrop"
          onClick={() => {
            setIsCreateDialogOpen(false);
            setCreateMaterialError(undefined);
          }}
        >
          <div
            className="cylinder-weight-modal cylinder-weight-create-modal"
            role="dialog"
            aria-modal="true"
            aria-label="New material"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="cylinder-weight-modal-header">
              <h3>New Material</h3>
            </div>

            <div className="cylinder-weight-create-inline">
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

              {createMaterialError ? (
                <div className="cylinder-weight-material-hint">
                  {createMaterialError}
                </div>
              ) : null}

              <div className="cylinder-weight-export-actions">
                <Button
                  variant="secondary"
                  size="small"
                  onClick={() => {
                    setIsCreateDialogOpen(false);
                    setCreateMaterialError(undefined);
                  }}
                >
                  Cancel
                </Button>
                <Button
                  variant="primary"
                  size="small"
                  onClick={onCreateMaterial}
                >
                  Save
                </Button>
              </div>
            </div>
          </div>
        </div>
      ) : null}

      {importSummary ? (
        <div
          className="cylinder-weight-modal-backdrop"
          onClick={() => setImportSummary(null)}
        >
          <div
            className="cylinder-weight-modal cylinder-weight-export-modal"
            role="dialog"
            aria-modal="true"
            aria-label="Import completed"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="cylinder-weight-modal-header">
              <h3>Import completed</h3>
            </div>

            <div className="cylinder-weight-import-summary">
              <p>Imported {importSummary.imported} materials.</p>
              <p>Skipped {importSummary.skippedDuplicates} duplicates.</p>
              <p>Skipped {importSummary.skippedInvalid} invalid materials.</p>
            </div>

            <div className="cylinder-weight-export-actions">
              <Button
                variant="primary"
                size="small"
                onClick={() => setImportSummary(null)}
              >
                OK
              </Button>
            </div>
          </div>
        </div>
      ) : null}

      {exportSummary ? (
        <div
          className="cylinder-weight-modal-backdrop"
          onClick={() => setExportSummary(null)}
        >
          <div
            className="cylinder-weight-modal cylinder-weight-export-modal"
            role="dialog"
            aria-modal="true"
            aria-label="Export completed"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="cylinder-weight-modal-header">
              <h3>Export completed</h3>
            </div>

            <div className="cylinder-weight-import-summary">
              <p>Exported {exportSummary.exported} materials successfully.</p>
            </div>

            <div className="cylinder-weight-export-actions">
              <Button
                variant="primary"
                size="small"
                onClick={() => setExportSummary(null)}
              >
                OK
              </Button>
            </div>
          </div>
        </div>
      ) : null}

      {isExportOpen ? (
        <div
          className="cylinder-weight-modal-backdrop"
          onClick={onCancelExportDialog}
        >
          <div
            className="cylinder-weight-modal cylinder-weight-export-modal"
            role="dialog"
            aria-modal="true"
            aria-label="Export materials"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="cylinder-weight-modal-header">
              <h3>Export materials</h3>
            </div>

            <p className="cylinder-weight-export-help">
              Choose which materials to include in the export file. This will create a JSON file
              containing only selected materials and download it to your computer. You can use the
              file as a backup or to move materials to another machine.
            </p>

            <label className="cylinder-weight-export-row cylinder-weight-export-select-all">
              <input
                ref={selectAllRef}
                type="checkbox"
                checked={allExportSelected}
                onChange={(e) => setExportAll(e.target.checked)}
              />
              <span>Select all</span>
            </label>

            <div className="cylinder-weight-export-list">
              {materials.map((material) => (
                <label key={material.id} className="cylinder-weight-export-row">
                  <input
                    type="checkbox"
                    checked={selectedExportIds.includes(material.id)}
                    onChange={() => toggleExportMaterial(material.id)}
                  />
                  <span className="cylinder-weight-export-name">{material.name}</span>
                  <span className="cylinder-weight-export-density">
                    {material.density_kg_m3} kg/m3
                  </span>
                </label>
              ))}
            </div>

            <div className="cylinder-weight-export-actions">
              <Button variant="secondary" size="small" onClick={onCancelExportDialog}>
                Cancel
              </Button>
              <Button
                variant="primary"
                size="small"
                onClick={onConfirmExportSelected}
                disabled={selectedExportIds.length === 0}
              >
                Export selected
              </Button>
            </div>
          </div>
        </div>
      ) : null}
    </>
  );
}

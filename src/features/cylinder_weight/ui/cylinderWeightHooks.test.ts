import { describe, expect, it } from "vitest";

import { machineField, userField } from "@shared/form/types/fields";

import { createInitialCylinderWeightForm } from "../domain/cylinderWeightForm";
import type { CylinderMaterial } from "./materials";
import {
  buildCylinderMaterialsExportPayload,
  buildCylinderMaterialsExportSummary,
  setVisibleExportSelection,
  toggleExportMaterialId,
} from "./useCylinderMaterialManagement";
import {
  applyDefaultMaterialSelection,
  applyMaterialSelection,
} from "./useCylinderMaterials";
import { resetCylinderWeightFormKeepingMaterial } from "./useCylinderWeightCalculation";

const steel: CylinderMaterial = {
  id: "steel",
  name: "Steel",
  density_kg_m3: 7850,
};

const aluminum: CylinderMaterial = {
  id: "aluminum",
  name: "Aluminum",
  density_kg_m3: 2700,
};

describe("cylinder weight hook helpers", () => {
  it("selects the first loaded material only when the form has no material", () => {
    const emptyForm = createInitialCylinderWeightForm();

    expect(applyDefaultMaterialSelection(emptyForm, [steel]).extras).toMatchObject({
      materialId: "steel",
      materialName: "Steel",
      densityKgM3: 7850,
    });

    const existingForm = {
      ...createInitialCylinderWeightForm(),
      extras: {
        materialId: "aluminum",
        materialName: "Aluminum",
        densityKgM3: 2700,
      },
    };

    expect(applyDefaultMaterialSelection(existingForm, [steel]).extras).toEqual(
      existingForm.extras,
    );
  });

  it("selecting material clears calculated fields and form errors", () => {
    const form = {
      ...createInitialCylinderWeightForm(),
      status: "solved" as const,
      fields: {
        ...createInitialCylinderWeightForm().fields,
        outer_diameter_mm: userField("20", { machineValue: 20 }),
        mass_kg: machineField("12.5", { machineValue: 12.5 }),
      },
      formError: "Old error",
    };

    const next = applyMaterialSelection(form, "aluminum", [steel, aluminum]);

    expect(next.status).toBe("editing");
    expect(next.formError).toBeUndefined();
    expect(next.extras).toMatchObject({
      materialId: "aluminum",
      materialName: "Aluminum",
      densityKgM3: 2700,
    });
    expect(next.fields.outer_diameter_mm.value).toBe("20");
    expect(next.fields.outer_diameter_mm.machineValue).toBe(20);
    expect(next.fields.mass_kg.value).toBe("");
    expect(next.fields.mass_kg.source).toBe("empty");
  });

  it("reset keeps selected material but clears cylinder fields", () => {
    const form = {
      ...createInitialCylinderWeightForm(),
      fields: {
        ...createInitialCylinderWeightForm().fields,
        outer_diameter_mm: userField("20"),
        mass_kg: machineField("12.5", { machineValue: 12.5 }),
      },
      extras: {
        materialId: "steel",
        materialName: "Steel",
        densityKgM3: 7850,
      },
    };

    const next = resetCylinderWeightFormKeepingMaterial(form);

    expect(next.extras).toEqual(form.extras);
    expect(next.fields.outer_diameter_mm.value).toBe("");
    expect(next.fields.mass_kg.value).toBe("");
    expect(next.status).toBe("editing");
  });

  it("updates export selection predictably", () => {
    expect(toggleExportMaterialId(["steel"], "aluminum")).toEqual([
      "steel",
      "aluminum",
    ]);
    expect(toggleExportMaterialId(["steel", "aluminum"], "steel")).toEqual([
      "aluminum",
    ]);
    expect(setVisibleExportSelection(["steel"], true, ["steel", "aluminum"])).toEqual([
      "steel",
      "aluminum",
    ]);
    expect(setVisibleExportSelection(["steel", "aluminum"], false, ["steel"])).toEqual([
      "aluminum",
    ]);
  });

  it("builds export payload and summary from selected materials", () => {
    const selected = [steel, aluminum];

    expect(buildCylinderMaterialsExportPayload(selected)).toEqual({
      schema_version: 1,
      materials: [
        { name: "Steel", density_kg_m3: 7850 },
        { name: "Aluminum", density_kg_m3: 2700 },
      ],
    });
    expect(buildCylinderMaterialsExportSummary(selected)).toEqual({
      exported: 2,
      materials: [
        { name: "Steel", density_kg_m3: 7850 },
        { name: "Aluminum", density_kg_m3: 2700 },
      ],
    });
  });
});

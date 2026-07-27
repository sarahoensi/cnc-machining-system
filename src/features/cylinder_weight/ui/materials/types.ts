// src/features/cylinder_weight/ui/materials/types.ts

export type CylinderMaterial = {
  id: string;
  name: string;
  density_kg_m3: number;
};

export type ImportSummary = {
  imported: number;
  skippedDuplicates: number;
  skippedInvalid: number;
  added: Array<{
    name: string;
    density_kg_m3: number;
    original_name?: string;
  }>;
  skipped: Array<{
    name?: string;
    density_kg_m3?: number;
    reason: "duplicate" | "invalid";
    message: string;
  }>;
};

export type ExportSummary = {
  exported: number;
  materials: Array<{
    name: string;
    density_kg_m3: number;
  }>;
};

export type MaterialEditState = {
  id: string;
  name: string;
  setName: (value: string) => void;
  density: string;
  setDensity: (value: string) => void;
  error?: string;
  start: (material: CylinderMaterial) => void;
  cancel: () => void;
  save: () => void;
  remove: (id: string) => void;
};

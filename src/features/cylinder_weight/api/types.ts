// src/features/cylinder_weight/api/types.ts

export type SolveCylinderWeightRequest = {
  material_id?: string;
  outer_diameter_mm?: number;
  inner_diameter_mm?: number;
  length_mm?: number;
};

export type SolveCylinderWeightResponse = {
  material_name: string;
  density_kg_m3: number;
  outer_diameter_mm: number;
  inner_diameter_mm: number;
  length_mm: number;
  mass_kg: number;
};

export type CreateCylinderMaterialRequest = {
  name?: string;
  density_kg_m3?: number;
};

export type UpdateCylinderMaterialRequest = {
  id?: string;
  name?: string;
  density_kg_m3?: number;
};

export type DeleteCylinderMaterialRequest = {
  id?: string;
};

export type CylinderMaterialResponse = {
  id: string;
  name: string;
  density_kg_m3: number;
};

export type ImportCylinderMaterialsRequest = {
  json_payload?: string;
};

export type ImportCylinderMaterialsResponse = {
  imported: number;
  skipped_duplicates: number;
  skipped_invalid: number;
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

export type ExportCylinderMaterialResponse = {
  name: string;
  density_kg_m3: number;
};

export type ExportCylinderMaterialsResponse = {
  schema_version: number;
  materials: ExportCylinderMaterialResponse[];
};


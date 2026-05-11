import { tauriInvoke } from "@shared/api/tauriClient";
import {
  CreateCylinderMaterialRequest,
  CylinderMaterialResponse,
  DeleteCylinderMaterialRequest,
  ExportCylinderMaterialsResponse,
  ImportCylinderMaterialsRequest,
  ImportCylinderMaterialsResponse,
  SolveCylinderWeightRequest,
  SolveCylinderWeightResponse,
  UpdateCylinderMaterialRequest,
} from "./types";

export function listCylinderMaterialsApi() {
  return tauriInvoke<CylinderMaterialResponse[]>(
    "list_cylinder_materials"
  );
}

export function createCylinderMaterialApi(
  request: CreateCylinderMaterialRequest
) {
  return tauriInvoke<CylinderMaterialResponse>(
    "create_cylinder_material",
    { request }
  );
}

export function updateCylinderMaterialApi(
  request: UpdateCylinderMaterialRequest
) {
  return tauriInvoke<CylinderMaterialResponse>(
    "update_cylinder_material",
    { request }
  );
}

export function deleteCylinderMaterialApi(
  request: DeleteCylinderMaterialRequest
) {
  return tauriInvoke<void>(
    "delete_cylinder_material",
    { request }
  );
}

export function solveCylinderWeightApi(
  request: SolveCylinderWeightRequest
) {
  return tauriInvoke<SolveCylinderWeightResponse>(
    "solve_cylinder_weight",
    { request }
  );
}

export function importCylinderMaterialsApi(
  request: ImportCylinderMaterialsRequest
) {
  return tauriInvoke<ImportCylinderMaterialsResponse>(
    "import_cylinder_materials",
    { request }
  );
}

export function exportCylinderMaterialsApi() {
  return tauriInvoke<ExportCylinderMaterialsResponse>(
    "export_cylinder_materials"
  );
}

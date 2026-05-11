import { tauriInvoke } from "@shared/api/tauriClient";
import {
  CreateCylinderMaterialRequest,
  CylinderMaterialResponse,
  DeleteCylinderMaterialRequest,
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

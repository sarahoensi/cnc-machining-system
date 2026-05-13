import { CylinderMaterial } from "./types";

export function matchesMaterialSearch(materialName: string, search: string) {
  const normalizedQuery = search.trim().toLowerCase();

  if (!normalizedQuery) {
    return true;
  }

  return materialName
    .toLowerCase()
    .split(/\s+/)
    .some((token) => token.startsWith(normalizedQuery));
}

export function filterMaterialsBySearch(materials: CylinderMaterial[], search: string) {
  return materials.filter((material) => matchesMaterialSearch(material.name, search));
}

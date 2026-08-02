// src/features/cylinder_weight/ui/materials/export/MaterialExportTable.tsx

import { Table } from "@shared/ui/table/Table";
import { CylinderMaterial } from "../types";

type Props = {
  materials: CylinderMaterial[];
  selectedIds: string[];
  onToggle: (id: string) => void;
};

export function MaterialExportTable({ materials, selectedIds, onToggle }: Props) {
  if (materials.length === 0) {
    return <p className="cylinder-weight-material-empty">No materials found</p>;
  }

  return (
    <Table.Root className="cylinder-weight-export-materials-table">
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell className="cylinder-weight-export-materials-check-col" />
          <Table.HeaderCell>Material</Table.HeaderCell>
          <Table.HeaderCell
            align="right"
            className="cylinder-weight-export-materials-density-col"
          >
            Density
          </Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>
      <Table.Body>
        {materials.map((material) => (
          <Table.BodyRow key={material.id}>
            <Table.Cell className="cylinder-weight-export-materials-check-col">
              <input
                type="checkbox"
                checked={selectedIds.includes(material.id)}
                onChange={() => onToggle(material.id)}
              />
            </Table.Cell>
            <Table.Cell>{material.name}</Table.Cell>
            <Table.Cell
              align="right"
              className="cylinder-weight-export-materials-density-col"
            >
              {material.density_kg_m3} kg/m3
            </Table.Cell>
          </Table.BodyRow>
        ))}
      </Table.Body>
    </Table.Root>
  );
}

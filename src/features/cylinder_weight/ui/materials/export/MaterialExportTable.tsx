import { useEffect, useRef } from "react";
import { Table } from "@shared/ui/components/table/Table";
import { CylinderMaterial } from "../types";

type Props = {
  materials: CylinderMaterial[];
  selectedIds: string[];
  onSetAll: (checked: boolean) => void;
  onToggle: (id: string) => void;
};

export function MaterialExportTable({
  materials,
  selectedIds,
  onSetAll,
  onToggle,
}: Props) {
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = materials.length > 0 && selectedIds.length === materials.length;
  const someSelected = selectedIds.length > 0 && selectedIds.length < materials.length;

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someSelected;
    }
  }, [someSelected]);

  return (
    <Table.Root>
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>
            <label className="cylinder-weight-checkbox-label">
              <input
                ref={selectAllRef}
                type="checkbox"
                checked={allSelected}
                onChange={(e) => onSetAll(e.target.checked)}
              />
              <span>Select all</span>
            </label>
          </Table.HeaderCell>
          <Table.HeaderCell>Material</Table.HeaderCell>
          <Table.HeaderCell align="right">Density</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>
      <Table.Body>
        {materials.map((material) => (
          <Table.BodyRow key={material.id}>
            <Table.Cell>
              <input
                type="checkbox"
                checked={selectedIds.includes(material.id)}
                onChange={() => onToggle(material.id)}
              />
            </Table.Cell>
            <Table.Cell>{material.name}</Table.Cell>
            <Table.Cell align="right">{material.density_kg_m3} kg/m3</Table.Cell>
          </Table.BodyRow>
        ))}
      </Table.Body>
    </Table.Root>
  );
}

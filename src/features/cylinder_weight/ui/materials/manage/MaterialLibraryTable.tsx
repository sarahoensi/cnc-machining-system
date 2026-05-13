// src/features/cylinder_weight/ui/materials/manage/MaterialLibraryTable.tsx

import { Table } from "@shared/ui/components/table/Table";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/input";
import { TextInput } from "@shared/ui/primitives/input";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { CylinderMaterial, MaterialEditState } from "../types";
import { sortCylinderMaterials } from "../sortMaterials";

type Props = {
  materials: CylinderMaterial[];
  edit: MaterialEditState;
};

export function MaterialLibraryTable({ materials, edit }: Props) {
  const sortedMaterials = sortCylinderMaterials(materials);

  if (sortedMaterials.length === 0) {
    return <p className="cylinder-weight-material-empty">No materials found</p>;
  }

  return (
    <Table.Root className="cylinder-weight-material-table">
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>Material</Table.HeaderCell>
          <Table.HeaderCell align="right">Density</Table.HeaderCell>
          <Table.HeaderCell align="right">Actions</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>

      <Table.Body>
        {sortedMaterials.map((material) => (
          <MaterialRow key={material.id} material={material} edit={edit} />
        ))}
      </Table.Body>
    </Table.Root>
  );
}

function MaterialRow({
  material,
  edit,
}: {
  material: CylinderMaterial;
  edit: MaterialEditState;
}) {
  if (edit.id === material.id) {
    return <EditableMaterialRow edit={edit} />;
  }

  return <ReadonlyMaterialRow material={material} edit={edit} />;
}

function EditableMaterialRow({ edit }: { edit: MaterialEditState }) {
  return (
    <Table.BodyRow>
      <Table.Cell>
        <TextInput
          value={edit.name}
          onChange={edit.setName}
          appearance="form"
          source="user"
          size="small"
        />
      </Table.Cell>

      <Table.Cell align="right">
        <NumberInput
          value={edit.density}
          onChange={edit.setDensity}
          unit="kg/m3"
          appearance="form"
          source="user"
          size="small"
        />
      </Table.Cell>

      <Table.Cell align="right">
        <DialogActions>
          <Button variant="link" onClick={edit.save}>
            Save
          </Button>

          <Button variant="link" onClick={edit.cancel}>
            Cancel
          </Button>
        </DialogActions>
      </Table.Cell>
    </Table.BodyRow>
  );
}

function ReadonlyMaterialRow({
  material,
  edit,
}: {
  material: CylinderMaterial;
  edit: MaterialEditState;
}) {
  return (
    <Table.BodyRow>
      <Table.Cell>{material.name}</Table.Cell>
      <Table.Cell align="right">{material.density_kg_m3} kg/m3</Table.Cell>

      <Table.Cell align="right">
        <DialogActions>
          <Button variant="link" onClick={() => edit.start(material)}>
            Edit
          </Button>

          <Button variant="danger" size="small" onClick={() => edit.remove(material.id)}>
            Delete
          </Button>
        </DialogActions>
      </Table.Cell>
    </Table.BodyRow>
  );
}



import { Table } from "@shared/ui/components/table/Table";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/NumberInput/NumberInput";
import { TextInput } from "@shared/ui/primitives/TextInput/TextInput";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { CylinderMaterial, MaterialEditState } from "../types";

type Props = {
  materials: CylinderMaterial[];
  edit: MaterialEditState;
};

export function MaterialLibraryTable({ materials, edit }: Props) {
  return (
    <Table.Root>
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>Material</Table.HeaderCell>
          <Table.HeaderCell align="right">Density</Table.HeaderCell>
          <Table.HeaderCell align="right">Actions</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>

      <Table.Body>
        {materials.map((material) => (
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
        <TextInput value={edit.name} onChange={edit.setName} />
      </Table.Cell>

      <Table.Cell align="right">
        <NumberInput
          value={edit.density}
          onChange={edit.setDensity}
          unit="kg/m3"
          className="ni-form ni-user"
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

          <Button variant="link" onClick={() => edit.remove(material.id)}>
            Delete
          </Button>
        </DialogActions>
      </Table.Cell>
    </Table.BodyRow>
  );
}
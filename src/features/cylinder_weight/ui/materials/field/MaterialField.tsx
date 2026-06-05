// src/features/cylinder_weight/ui/materials/field/MaterialField.tsx

// MaterialField.tsx

import { FormError } from "@shared/ui/form/FormError";
import { FormSelectMenuField } from "@shared/ui/form/fields";
import { Button } from "@shared/ui/primitives/Button/Button";
import { SelectMenuLabel } from "@shared/ui/primitives/Select";
import { CylinderMaterial } from "../types";

type Props = {
  materials: CylinderMaterial[];
  selectedMaterial?: CylinderMaterial;
  onMaterialChange: (id: string) => void;
  onOpenManage: () => void;
  onOpenCreate: () => void;
  materialLoadError?: string;
};

export function MaterialField({
  materials,
  selectedMaterial,
  onMaterialChange,
  onOpenManage,
  onOpenCreate,
  materialLoadError,
}: Props) {
  return (
    <div className="cylinder-weight-material-field">
      <FormSelectMenuField
        label="Material"
        tooltip="Select material to use for density in mass calculation."
        valueLabel={
          selectedMaterial ? (
            <SelectMenuLabel
              label={selectedMaterial.name}
              meta={`${selectedMaterial.density_kg_m3} kg/m³`}
            />
          ) : (
            "Select material"
          )
        }
        options={materials.map((material) => ({
          value: material.id,
          label: material.name,
          meta: `${material.density_kg_m3} kg/m³`,
        }))}
        onSelect={onMaterialChange}
        utilityItems={[
          {
            label: "+ New Material...",
            onClick: () => {
              onOpenManage();
              onOpenCreate();
            },
          },
          {
            label: "Manage Materials...",
            onClick: () => {
              onOpenManage();
            },
          },
        ]}
      />

      <Button variant="link" className="cylinder-weight-manage-link" onClick={onOpenManage}>
        Manage Materials
      </Button>

      {materialLoadError ? <FormError error={materialLoadError} /> : null}
    </div>
  );
}



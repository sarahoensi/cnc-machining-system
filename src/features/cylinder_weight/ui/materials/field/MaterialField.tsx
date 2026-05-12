// src/features/cylinder_weight/ui/materials/field/MaterialField.tsx

// MaterialField.tsx

import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormSelectMenuField } from "@shared/ui/components/form/fields";
import { Button } from "@shared/ui/primitives/Button/Button";
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
        valueLabel={selectedMaterial?.name ?? "Select material"}
        options={materials.map((material) => ({
          value: material.id,
          label: material.name,
          meta: `\u00b7 ${material.density_kg_m3} kg/m3`,
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


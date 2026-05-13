// src/features/cylinder_weight/ui/materials/field/MaterialField.tsx

// MaterialField.tsx

import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormSelectMenuField } from "@shared/ui/components/form/fields";
import { Button } from "@shared/ui/primitives/Button/Button";
import { SelectMenuLabel } from "@shared/ui/primitives/Select";
import { Ref } from "react";
import { CylinderMaterial } from "../types";

type Props = {
  materials: CylinderMaterial[];
  selectedMaterial?: CylinderMaterial;
  onMaterialChange: (id: string) => void;
  onOpenManage: () => void;
  onOpenCreate: () => void;
  materialLoadError?: string;
  error?: string;
  triggerRef?: Ref<HTMLButtonElement>;
};

export function MaterialField({
  materials,
  selectedMaterial,
  onMaterialChange,
  onOpenManage,
  onOpenCreate,
  materialLoadError,
  error,
  triggerRef,
}: Props) {
  return (
    <div className="cylinder-weight-material-field">
      <FormSelectMenuField
        ref={triggerRef}
        label="Material"
        tooltip="Select material to use for density in mass calculation."
        error={error}
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


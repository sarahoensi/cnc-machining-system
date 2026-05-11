// MaterialField.tsx

import { useEffect, useRef, useState } from "react";
import { Field } from "@shared/ui/components/form/Field/Field";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { SelectMenu } from "@shared/ui/primitives/Select";
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
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleOutsideClick(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handleOutsideClick);
    return () => document.removeEventListener("mousedown", handleOutsideClick);
  }, []);

  return (
    <div className="cylinder-weight-material-field">
      <Field label="Material">
        <div ref={menuRef}>
          <SelectMenu
            valueLabel={selectedMaterial?.name ?? "Select material"}
            open={open}
            onToggle={() => setOpen((current) => !current)}
            options={materials.map((material) => ({
              value: material.id,
              label: material.name,
              meta: `\u00b7 ${material.density_kg_m3} kg/m3`,
            }))}
            onSelect={(id) => {
              onMaterialChange(id);
              setOpen(false);
            }}
            utilityItems={[
              {
                label: "+ New Material...",
                onClick: () => {
                  setOpen(false);
                  onOpenManage();
                  onOpenCreate();
                },
              },
              {
                label: "Manage Materials...",
                onClick: () => {
                  setOpen(false);
                  onOpenManage();
                },
              },
            ]}
          />
        </div>
      </Field>

      <Button variant="link" className="cylinder-weight-manage-link" onClick={onOpenManage}>
        Manage Materials
      </Button>

      {materialLoadError ? <FormError error={materialLoadError} /> : null}
    </div>
  );
}

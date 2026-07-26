// src/features/cylinder_weight/ui/materials/create/NewMaterialModal.tsx

import { useRef } from "react";
import { Field } from "@shared/ui/form/Field";
import { FormError } from "@shared/ui/form/FormError";
import { FormTextField } from "@shared/ui/form/fields";
import { Modal } from "@shared/ui/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/overlay/DialogActions/DialogActions";
import { Stack } from "@shared/ui/primitives/Stack/Stack";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/input";

type Props = {
  open: boolean;
  onClose: () => void;
  name: string;
  setName: (value: string) => void;
  density: string;
  setDensity: (value: string) => void;
  error?: string;
  onSave: () => void;
};

export function NewMaterialModal({
  open,
  onClose,
  name,
  setName,
  density,
  setDensity,
  error,
  onSave,
}: Props) {
  const densityInputRef = useRef<HTMLInputElement>(null);

  if (!open) return null;

  return (
    <Modal title="New Material" size="sm" onClose={onClose}>
      <Stack>
        <FormTextField
          label="Name"
          value={name}
          onChange={setName}
          source="user"
          placeholder="Ex: Bronze"
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              event.preventDefault();
              densityInputRef.current?.focus();
            }
          }}
        />

        <Field label="Density">
          <NumberInput
            ref={densityInputRef}
            value={density}
            onChange={setDensity}
            unit="kg/m3"
            appearance="form"
            source="user"
            placeholder="Ex: 8800"
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onSave();
              }
            }}
          />
        </Field>

        {error ? <FormError error={error} /> : null}

        <DialogActions>
          <Button variant="secondary" size="small" onClick={onClose}>
            Cancel
          </Button>

          <Button variant="primary" size="small" onClick={onSave}>
            Save
          </Button>
        </DialogActions>
      </Stack>
    </Modal>
  );
}




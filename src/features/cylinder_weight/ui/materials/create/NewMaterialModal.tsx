import { Field } from "@shared/ui/components/form/Field/Field";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { Modal } from "@shared/ui/components/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { FormStack } from "@shared/ui/layout/container/FormStack/FormStack";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/NumberInput/NumberInput";
import { TextInput } from "@shared/ui/primitives/TextInput/TextInput";

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
  if (!open) return null;

  return (
    <Modal title="New Material" size="sm" onClose={onClose}>
      <FormStack>
        <Field label="Name">
          <TextInput value={name} onChange={setName} placeholder="Ex: Bronze" />
        </Field>

        <Field label="Density">
          <NumberInput
            value={density}
            onChange={setDensity}
            unit="kg/m3"
            className="ni-form ni-user"
            placeholder="Ex: 8800"
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
      </FormStack>
    </Modal>
  );
}
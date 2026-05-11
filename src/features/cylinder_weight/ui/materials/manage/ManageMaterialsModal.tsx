//ManageMaterialsModal.tsx

import { useRef } from "react";
import { Modal, ModalScrollArea } from "@shared/ui/components/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { Button } from "@shared/ui/primitives/Button/Button";
import { CylinderMaterial, MaterialEditState } from "../types";
import { MaterialLibraryTable } from "./MaterialLibraryTable";

type Props = {
  open: boolean;
  onClose: () => void;
  materials: CylinderMaterial[];
  onOpenCreate: () => void;
  onOpenExport: () => void;
  onImportFile: (file: File) => void;
  edit: MaterialEditState;
};

export function ManageMaterialsModal({
  open,
  onClose,
  materials,
  onOpenCreate,
  onOpenExport,
  onImportFile,
  edit,
}: Props) {
  const importInputRef = useRef<HTMLInputElement>(null);

  if (!open) return null;

  return (
    <Modal title="Manage Materials" onClose={onClose} size="md" height="fixed">
      <DialogActions align="left">
        <Button variant="secondary" size="small" onClick={onOpenCreate}>
          + New Material
        </Button>

        <Button
          variant="secondary"
          size="small"
          onClick={() => importInputRef.current?.click()}
        >
          Import Materials
        </Button>

        <Button variant="secondary" size="small" onClick={onOpenExport}>
          Export Materials
        </Button>
      </DialogActions>

      <input
        ref={importInputRef}
        type="file"
        accept="application/json,.json"
        hidden
        onChange={(event) => {
          const file = event.target.files?.[0];

          if (file) {
            onImportFile(file);
          }

          event.currentTarget.value = "";
        }}
      />

      <ModalScrollArea>
        <MaterialLibraryTable materials={materials} edit={edit} />
      </ModalScrollArea>

      {edit.error ? <FormError error={edit.error} /> : null}
    </Modal>
  );
}

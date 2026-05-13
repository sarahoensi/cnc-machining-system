// src/features/cylinder_weight/ui/materials/manage/ManageMaterialsModal.tsx

//ManageMaterialsModal.tsx

import { useMemo, useRef, useState } from "react";
import { Modal, ModalScrollArea } from "@shared/ui/components/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { Button } from "@shared/ui/primitives/Button/Button";
import { TextInput } from "@shared/ui/primitives/input";
import SearchIcon from "@assets/search-icon.svg";
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
  const [search, setSearch] = useState("");

  const filteredMaterials = useMemo(() => {
    const normalizedQuery = search.trim().toLowerCase();

    if (!normalizedQuery) {
      return materials;
    }

    return materials.filter((material) =>
      material.name
        .toLowerCase()
        .split(/\s+/)
        .some((token) => token.startsWith(normalizedQuery))
    );
  }, [materials, search]);

  if (!open) return null;

  return (
    <Modal title="Manage Materials" onClose={onClose} size="md" height="fixed">
      <div className="cylinder-weight-toolbar">
        <div className="cylinder-weight-toolbar-left">
          <Button variant="primary" size="small" onClick={onOpenCreate}>
            + New Material
          </Button>

          <div className="cylinder-weight-material-search">
            <TextInput
              value={search}
              onChange={setSearch}
              placeholder="Search materials"
              appearance="form"
              size="small"
              className="cylinder-weight-material-search-input"
              leftSlot={<img src={SearchIcon} alt="" className="cylinder-weight-search-icon" />}
            />
          </div>
        </div>

        <DialogActions align="right">
          <Button
            variant="link"
            size="small"
            onClick={() => importInputRef.current?.click()}
          >
            Import
          </Button>

          <Button variant="link" size="small" onClick={onOpenExport}>
            Export
          </Button>
        </DialogActions>
      </div>

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
        <MaterialLibraryTable materials={filteredMaterials} edit={edit} />
      </ModalScrollArea>

      {edit.error ? <FormError error={edit.error} /> : null}
    </Modal>
  );
}


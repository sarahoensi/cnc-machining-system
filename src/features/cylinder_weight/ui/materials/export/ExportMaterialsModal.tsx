// src/features/cylinder_weight/ui/materials/export/ExportMaterialsModal.tsx

import { Modal } from "@shared/ui/components/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/components/overlay/DialogActions/DialogActions";
import { Button } from "@shared/ui/primitives/Button/Button";
import { MaterialExportTable } from "./MaterialExportTable";
import { CylinderMaterial } from "../types";

type Props = {
  open: boolean;
  onClose: () => void;
  materials: CylinderMaterial[];
  selectedIds: string[];
  onSetAll: (checked: boolean) => void;
  onToggle: (id: string) => void;
  onConfirm: () => void;
};

export function ExportMaterialsModal({
  open,
  onClose,
  materials,
  selectedIds,
  onSetAll,
  onToggle,
  onConfirm,
}: Props) {
  if (!open) return null;

  return (
    <Modal title="Export materials" size="md" onClose={onClose} showCloseButton={false}>
      <p className="cylinder-weight-export-help">
        Choose which materials to include in the export file. This will create a JSON file
        containing only selected materials and download it to your computer. You can use the
        file as a backup or to move materials to another machine.
      </p>

      <MaterialExportTable
        materials={materials}
        selectedIds={selectedIds}
        onSetAll={onSetAll}
        onToggle={onToggle}
      />

      <DialogActions>
        <Button variant="secondary" size="small" onClick={onClose}>
          Cancel
        </Button>
        <Button
          variant="primary"
          size="small"
          onClick={onConfirm}
          disabled={selectedIds.length === 0}
        >
          Export selected
        </Button>
      </DialogActions>
    </Modal>
  );
}


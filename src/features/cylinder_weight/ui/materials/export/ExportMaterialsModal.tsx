// src/features/cylinder_weight/ui/materials/export/ExportMaterialsModal.tsx

import { useEffect, useMemo, useRef, useState } from "react";
import { Modal, ModalScrollArea } from "@shared/ui/overlay/Modal/Modal";
import { DialogActions } from "@shared/ui/overlay/DialogActions/DialogActions";
import { Button } from "@shared/ui/primitives/Button/Button";
import { TextInput } from "@shared/ui/primitives/input";
import SearchIcon from "@assets/search-icon.svg";
import { MaterialExportTable } from "./MaterialExportTable";
import { CylinderMaterial } from "../types";
import { filterMaterialsBySearch } from "../searchMaterials";

type Props = {
  open: boolean;
  onClose: () => void;
  materials: CylinderMaterial[];
  selectedIds: string[];
  onSetAll: (checked: boolean, visibleIds: string[]) => void;
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
  const selectAllRef = useRef<HTMLInputElement>(null);
  const [search, setSearch] = useState("");
  const filteredMaterials = useMemo(
    () => filterMaterialsBySearch(materials, search),
    [materials, search]
  );
  const visibleIds = useMemo(() => filteredMaterials.map((material) => material.id), [filteredMaterials]);
  const selectedVisibleCount = useMemo(
    () => visibleIds.filter((id) => selectedIds.includes(id)).length,
    [selectedIds, visibleIds]
  );
  const allVisibleSelected = visibleIds.length > 0 && selectedVisibleCount === visibleIds.length;
  const someVisibleSelected = selectedVisibleCount > 0 && selectedVisibleCount < visibleIds.length;

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someVisibleSelected;
    }
  }, [someVisibleSelected]);

  if (!open) return null;

  return (
    <Modal
      title="Export materials"
      size="md"
      height="fixed"
      onClose={onClose}
      showCloseButton={false}
    >
      <div className="cylinder-weight-export-content">
        <p className="cylinder-weight-export-help">
          Choose which materials to include in the export file. This will create a JSON file
          containing only selected materials and download it to your computer. You can use the
          file as a backup or to move materials to another machine.
        </p>

        <ModalScrollArea className="cylinder-weight-export-scroll-area">
          <div className="cylinder-weight-material-search cylinder-weight-export-search">
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

          <label className="cylinder-weight-checkbox-label cylinder-weight-export-select-all">
            <input
              ref={selectAllRef}
              type="checkbox"
              checked={allVisibleSelected}
              onChange={(e) => onSetAll(e.target.checked, visibleIds)}
            />
            <span>Select all visible</span>
          </label>

          <div className="cylinder-weight-export-table-area">
            <MaterialExportTable
              materials={filteredMaterials}
              selectedIds={selectedIds}
              onToggle={onToggle}
            />
          </div>
        </ModalScrollArea>
      </div>

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



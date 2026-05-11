import { ReactNode } from "react";
import { Modal } from "../Modal/Modal";
import { Button } from "@shared/ui/primitives/Button/Button";
import "./InfoDialog.css";

type InfoDialogProps = {
  open: boolean;
  title: string;
  onClose: () => void;
  children: ReactNode;
  confirmLabel?: string;
};

export function InfoDialog({
  open,
  title,
  onClose,
  children,
  confirmLabel = "OK",
}: InfoDialogProps) {
  if (!open) return null;

  return (
    <Modal
      title={title}
      size="sm"
      onClose={onClose}
      showCloseButton={false}
    >
      <div className="info-dialog-content">
        {children}
      </div>

      <div className="info-dialog-actions">
        <Button variant="primary" size="small" onClick={onClose}>
          {confirmLabel}
        </Button>
      </div>
    </Modal>
  );
}
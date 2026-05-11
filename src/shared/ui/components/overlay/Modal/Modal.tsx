import { ReactNode } from "react";
import clsx from "clsx";
import { Button } from "@shared/ui/primitives/Button/Button";
import "./Modal.css";

type ModalProps = {
  title: string;
  onClose: () => void;
  children: ReactNode;
  className?: string;
  closeLabel?: string;
  showCloseButton?: boolean;
  size?: "sm" | "md" | "lg";
  height?: "auto" | "fixed";
};

export function Modal({
  title,
  onClose,
  children,
  className,
  closeLabel = "Close",
  showCloseButton = true,
  size = "md",
  height = "auto",
}: ModalProps) {
  return (
    <div className="app-modal-backdrop" onClick={onClose}>
      <div
        className={clsx(
          "app-modal",
          `app-modal-${size}`,
          `app-modal-height-${height}`,
          className
        )}
        role="dialog"
        aria-modal="true"
        aria-label={title}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="app-modal-header">
          <h3>{title}</h3>
          {showCloseButton ? (
            <Button variant="secondary" size="small" onClick={onClose}>
              {closeLabel}
            </Button>
          ) : null}
        </div>

        {children}
      </div>
    </div>
  );
}

type ModalScrollAreaProps = {
  children: ReactNode;
  className?: string;
};

export function ModalScrollArea({ children, className }: ModalScrollAreaProps) {
  return <div className={clsx("app-modal-scroll-area", className)}>{children}</div>;
}

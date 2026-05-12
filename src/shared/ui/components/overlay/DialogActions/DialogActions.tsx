// shared/ui/components/overlay/DialogActions/DialogActions.tsx

import { ReactNode } from "react";
import "./DialogActions.css";

type DialogActionsAlign = "left" | "center" | "right";

type Props = {
  children: ReactNode;
  align?: DialogActionsAlign;
};

export function DialogActions({ children, align = "right" }: Props) {
  return <div className={`dialog-actions dialog-actions--${align}`}>{children}</div>;
}

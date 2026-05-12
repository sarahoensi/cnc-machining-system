// shared/ui/layout/container/FormStack/FormStack.tsx

import { ReactNode } from "react";
import "./FormStack.css";

export function FormStack({ children }: { children: ReactNode }) {
  return <div className="form-stack">{children}</div>;
}
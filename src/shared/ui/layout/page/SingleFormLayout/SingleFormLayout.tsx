import clsx from "clsx";
import type { ReactNode } from "react";
import "./SingleFormLayout.css";

type Props = {
  form: ReactNode;
  className?: string;
};

export function SingleFormLayout({ form, className }: Props) {
  return (
    <div className={clsx("single-form-layout", className)}>
      <div className="single-form-panel">{form}</div>
    </div>
  );
}

import clsx from "clsx";
import type { ReactNode } from "react";
import "./SplitFormLayout.css";

type SplitFormLayoutProps = {
  input: ReactNode;
  output: ReactNode;
  error?: ReactNode;
  actions: ReactNode;
  className?: string;
};

export function SplitFormLayout({
  input,
  output,
  error,
  actions,
  className,
}: SplitFormLayoutProps) {
  return (
    <div className={clsx("split-form-layout", className)}>
      <div className="split-form-input-panel">
        <div className="split-form-input">{input}</div>

        {error && <div className="split-form-error">{error}</div>}

        <div className="split-form-actions">{actions}</div>
      </div>

      <div className="split-form-output-panel">{output}</div>
    </div>
  );
}

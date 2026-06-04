import clsx from "clsx";
import type { CSSProperties, ReactNode } from "react";
import "./SplitFormLayout.css";

type SplitFormLayoutProps = {
  input: ReactNode;
  output: ReactNode;
  error?: ReactNode;
  actions: ReactNode;
  inputWidth?: string;
  outputWidth?: string;
  gap?: string;
  className?: string;
};

export function SplitFormLayout({
  input,
  output,
  error,
  actions,
  inputWidth,
  outputWidth,
  gap,
  className,
}: SplitFormLayoutProps) {
  const style = {
    "--split-form-input-width": inputWidth,
    "--split-form-output-width": outputWidth,
    "--split-form-gap": gap,
  } as CSSProperties;

  return (
    <div className={clsx("split-form-layout", className)} style={style}>
      <div className="split-form-input-panel">
        <div className="split-form-input">{input}</div>

        {error && <div className="split-form-error">{error}</div>}

        <div className="split-form-actions">{actions}</div>
      </div>

      <div className="split-form-output-panel">{output}</div>
    </div>
  );
}

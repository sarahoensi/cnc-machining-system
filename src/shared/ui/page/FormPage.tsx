import clsx from "clsx";
import type { CSSProperties, ReactNode } from "react";

import { PageShell } from "./PageShell";
import "./FormPage.css";

type Props = {
  form: ReactNode;
  panelWidth?: string;
  className?: string;
  formClassName?: string;
};

export function FormPage({
  form,
  panelWidth,
  className,
  formClassName,
}: Props) {
  const panelStyle = {
    "--form-page-panel-width": panelWidth,
  } as CSSProperties;

  return (
    <PageShell className={clsx("form-page", className)}>
      <div className={clsx("form-page-panel", formClassName)} style={panelStyle}>{form}</div>
    </PageShell>
  );
}

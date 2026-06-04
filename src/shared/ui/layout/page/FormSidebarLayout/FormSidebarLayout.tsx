// shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.tsx

import clsx from "clsx";
import type { ReactNode } from "react";
import "./FormSidebarLayout.css";

type Props = {
  form: ReactNode;
  sidebar: ReactNode;
  variant?: "default" | "compact";

  className?: string;

  formClassName?: string;
  sidebarClassName?: string;
};

export function FormSidebarLayout({
  form,
  sidebar,
  variant = "default",
  className,
  formClassName,
  sidebarClassName,
}: Props) {
  return (
    <div
      className={clsx(
        "form-sidebar-layout",
        variant === "compact" && "form-sidebar-layout--compact",
        className
      )}
    >
      <div className={clsx("fsl-form", formClassName)}>
        {form}
      </div>

      <aside className={clsx("fsl-sidebar", sidebarClassName)}>
        {sidebar}
      </aside>
    </div>
  );
}

// shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.tsx

import clsx from "clsx";
import type { ReactNode } from "react";
import {
  formWidthClassName,
  type FormWidth,
} from "../formWidth";
import "../formWidth.css";
import "./FormSidebarLayout.css";

type Props = {
  form: ReactNode;
  sidebar: ReactNode;
  formWidth?: FormWidth;
  variant?: "default" | "compact";

  className?: string;

  formClassName?: string;
  sidebarClassName?: string;
};

export function FormSidebarLayout({
  form,
  sidebar,
  formWidth = "sm",
  variant = "default",
  className,
  formClassName,
  sidebarClassName,
}: Props) {
  return (
    <div
      className={clsx(
        "form-sidebar-layout",
        formWidthClassName(formWidth),
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

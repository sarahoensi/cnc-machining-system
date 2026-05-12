// shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.tsx

import clsx from "clsx";
import "./FormSidebarLayout.css";

type Props = {
  form: React.ReactNode;
  sidebar: React.ReactNode;

  className?: string;

  formClassName?: string;
  sidebarClassName?: string;
};

export function FormSidebarLayout({
  form,
  sidebar,
  className,
  formClassName,
  sidebarClassName,
}: Props) {
  return (
    <div className={clsx("form-sidebar-layout", className)}>
      <div className={clsx("fsl-form", formClassName)}>
        {form}
      </div>

      <aside className={clsx("fsl-sidebar", sidebarClassName)}>
        {sidebar}
      </aside>
    </div>
  );
}
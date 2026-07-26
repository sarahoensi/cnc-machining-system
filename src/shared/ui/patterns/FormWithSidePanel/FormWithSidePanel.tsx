import clsx from "clsx";
import type { ReactNode } from "react";

import { PageShell } from "@shared/ui/page/PageShell";
import { Split } from "@shared/ui/primitives/Split/Split";

type Props = {
  form: ReactNode;
  sidePanel: ReactNode;
  primaryWidth?: string;
  secondaryWidth?: string;
  align?: "start" | "stretch";
  fillHeight?: boolean;
  secondaryMinHeightOnCollapse?: string;
  className?: string;
  formClassName?: string;
  sidePanelClassName?: string;
};

export function FormWithSidePanel({
  form,
  sidePanel,
  primaryWidth = "200px",
  secondaryWidth,
  align,
  fillHeight,
  secondaryMinHeightOnCollapse,
  className,
  formClassName,
  sidePanelClassName,
}: Props) {
  return (
    <PageShell className={clsx("form-with-side-panel", className)}>
      <Split
        primaryWidth={primaryWidth}
        secondaryWidth={secondaryWidth}
        align={align}
        fillHeight={fillHeight}
        secondaryMinHeightOnCollapse={secondaryMinHeightOnCollapse}
        primary={form}
        secondary={sidePanel}
        primaryClassName={formClassName}
        secondaryClassName={sidePanelClassName}
      />
    </PageShell>
  );
}

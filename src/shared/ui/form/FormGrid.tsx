import clsx from "clsx";
import type { CSSProperties, ReactNode } from "react";

import "./FormGrid.css";

type AreaRow = readonly string[];

type FormGridProps = {
  children: ReactNode;
  areas: readonly AreaRow[];
  columns?: string;
  gap?: string;
  rowGap?: string;
  columnGap?: string;
  align?: "start" | "center" | "end" | "stretch";
  collapse?: boolean;
  collapsedAreas?: readonly AreaRow[];
  className?: string;
};

type FormGridAreaProps = {
  name: string;
  children: ReactNode;
  className?: string;
};

function toGridAreas(areas: readonly AreaRow[]) {
  return areas.map((row) => `"${row.join(" ")}"`).join(" ");
}

function FormGridRoot({
  children,
  areas,
  columns = "1fr",
  gap,
  rowGap,
  columnGap,
  align = "stretch",
  collapse = true,
  collapsedAreas,
  className,
}: FormGridProps) {
  const style = {
    "--form-grid-areas": toGridAreas(areas),
    "--form-grid-collapsed-areas": toGridAreas(
      collapsedAreas ?? areas.flat().map((area) => [area]),
    ),
    "--form-grid-columns": columns,
    "--form-grid-gap": gap,
    "--form-grid-row-gap": rowGap,
    "--form-grid-column-gap": columnGap,
    "--form-grid-align": align,
  } as CSSProperties;

  return (
    <div
      className={clsx("form-grid", collapse && "form-grid--collapse", className)}
      style={style}
    >
      {children}
    </div>
  );
}

function FormGridArea({ name, children, className }: FormGridAreaProps) {
  const style = {
    gridArea: name,
  } as CSSProperties;

  return (
    <div className={clsx("form-grid-area", className)} style={style}>
      {children}
    </div>
  );
}

export const FormGrid = Object.assign(FormGridRoot, {
  Area: FormGridArea,
});

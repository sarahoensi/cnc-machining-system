import clsx from "clsx";
import type { CSSProperties, ReactNode } from "react";

import "./Split.css";

type Props = {
  primary: ReactNode;
  secondary: ReactNode;
  className?: string;
  primaryClassName?: string;
  secondaryClassName?: string;
  primaryWidth?: string;
  secondaryWidth?: string;
  gap?: string;
  align?: "start" | "stretch";
  fillHeight?: boolean;
  secondaryMinHeightOnCollapse?: string;
};

export function Split({
  primary,
  secondary,
  className,
  primaryClassName,
  secondaryClassName,
  primaryWidth,
  secondaryWidth,
  gap,
  align = "start",
  fillHeight = false,
  secondaryMinHeightOnCollapse,
}: Props) {
  const style = {
    "--split-primary-width": primaryWidth,
    "--split-secondary-width": secondaryWidth,
    "--split-gap": gap,
    "--split-secondary-min-height-on-collapse": secondaryMinHeightOnCollapse,
  } as CSSProperties;

  return (
    <div
      className={clsx(
        "split",
        align === "stretch" && "split--align-stretch",
        fillHeight && "split--fill-height",
        secondaryMinHeightOnCollapse && "split--secondary-min-height-on-collapse",
        className,
      )}
      style={style}
    >
      <div className={clsx("split-primary", primaryClassName)}>{primary}</div>
      <div className={clsx("split-secondary", secondaryClassName)}>{secondary}</div>
    </div>
  );
}

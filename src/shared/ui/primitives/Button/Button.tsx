// src/ui/components/Button/Button.tsx

import React, { forwardRef } from "react";
import clsx from "clsx";
import "./Button.css";

/* =====================================================
   TYPES
===================================================== */

export type ButtonVariant = "primary" | "secondary" | "danger" | "icon" | "link";

export type ButtonSize = "small" | "medium" | "large" | "icon";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
  size?: ButtonSize;
};

/* =====================================================
   BASE BUTTON
===================================================== */

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  { variant = "primary", size = "medium", className, children, ...rest },
  ref,
) {
  return (
    <button
      ref={ref}
      className={clsx(
        "app-button",
        `variant-${variant}`,
        variant !== "link" && `size-${size}`,
        size === "icon" && "size-icon",
        className,
      )}
      {...rest}
    >
      {children}
    </button>
  );
});

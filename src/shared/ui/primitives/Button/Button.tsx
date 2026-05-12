// src/ui/components/Button/Button.tsx

import React, { forwardRef } from "react";
import clsx from "clsx";
import "./Button.css";

import SettingsIcon from "@assets/settings-icon.svg";


/* =====================================================
   TYPES
===================================================== */

export type ButtonVariant =
  | "primary"
  | "secondary"
  | "danger"
  | "icon"
  | "link";

export type ButtonSize =
  | "small"
  | "medium"
  | "large"
  | "icon";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
  size?: ButtonSize;
};

/* =====================================================
   BASE BUTTON
===================================================== */

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  function Button(
    {
      variant = "primary",
      size = "medium",
      className,
      children,
      ...rest
    },
    ref
  ) {
    return (
      <button
        ref={ref}
        className={clsx(
          "app-button",
          `variant-${variant}`,
          variant !== "link" && `size-${size}`,
          size === "icon" && "size-icon",
          className
        )}
        {...rest}
      >
        {children}
      </button>
    );
  }
);

/* =====================================================
   PRESET BUTTONS
===================================================== */

type NativeButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement>;

export const CalculateButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="primary" size="large" {...props}>
      Calculate
    </Button>
  )
);

export const GenerateButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="primary" size="large" {...props}>
      Generate
    </Button>
  )
);

export const ResetButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="danger" size="medium" {...props}>
      Clear form
    </Button>
  )
);

export const RegisterButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="primary" size="small" {...props}>
      Registrer
    </Button>
  )
);

export const EditButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="secondary" size="small" {...props}>
      Edit
    </Button>
  )
);

export const OkButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="primary" size="small" {...props}>
      OK
    </Button>
  )
);

export const CancelButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="secondary" size="small" {...props}>
      Cancel
    </Button>
  )
);

export const SettingsButton = forwardRef<HTMLButtonElement, NativeButtonProps>(
  (props, ref) => (
    <Button ref={ref} variant="icon" size="icon" {...props}>
      <img
        src={SettingsIcon}
        alt="settings"
        className="icon-img"
      />
    </Button>
  )
);
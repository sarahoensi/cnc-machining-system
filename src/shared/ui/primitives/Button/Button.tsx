// src/ui/components/Button/Button.tsx

import React, { forwardRef } from "react";
import clsx from "clsx";
import "./Button.css";

import SettingsIcon from "../../../../assets/settings-icon.svg";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "danger" | "icon";
  size?: "small" | "medium" | "large" | "icon";
};

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
          variant,
          size,
          className
        )}
        {...rest}
      >
        {children}
      </button>
    );
  }
);

/* -------------------------------------------------- */
/* PRESET BUTTONS                                     */
/* -------------------------------------------------- */

export const CalculateButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function CalculateButton(props, ref) {
  return (
    <Button ref={ref} variant="primary" size="large" {...props}>
      Beregn
    </Button>
  );
});

export const ResetButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function ResetButton(props, ref) {
  return (
    <Button ref={ref} variant="danger" size="small" {...props}>
      Nullstill
    </Button>
  );
});

export const RegisterButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function RegisterButton(props, ref) {
  return (
    <Button ref={ref} variant="primary" size="small" {...props}>
      Registrer
    </Button>
  );
});

export const EditButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function EditButton(props, ref) {
  return (
    <Button ref={ref} variant="secondary" size="small" {...props}>
      Edit
    </Button>
  );
});

export const SettingsButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function SettingsButton(props, ref) {
  return (
    <Button ref={ref} variant="icon" size="icon" {...props}>
      <img src={SettingsIcon} alt="settings" className="icon-img" />
    </Button>
  );
});

export const OkButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function OkButton(props, ref) {
  return (
    <Button ref={ref} variant="primary" size="small" {...props}>
      OK
    </Button>
  );
});

export const CancelButton = forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement>
>(function CancelButton(props, ref) {
  return (
    <Button ref={ref} variant="secondary" size="small" {...props}>
      Avbryt
    </Button>
  );
});
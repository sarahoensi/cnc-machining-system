// src/shared/ui/primitives/input/TextInput/TextInput.tsx

import { FocusEventHandler, KeyboardEventHandler, forwardRef, useId } from "react";
import clsx from "clsx";
import { InputBase } from "@shared/ui/primitives/input/InputBase";
import type {
  InputAppearance,
  InputSize,
  InputSource,
} from "@shared/ui/primitives/input/types";
import "@shared/ui/primitives/input/InputControl/InputControl.css";
import "./TextInput.css";

type Props = {
  id?: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  autoFocus?: boolean;
  className?: string;
  onKeyDown?: KeyboardEventHandler<HTMLInputElement>;
  onFocus?: FocusEventHandler<HTMLInputElement>;
  onBlur?: FocusEventHandler<HTMLInputElement>;
  size?: InputSize;
  source?: InputSource;
  appearance?: InputAppearance;
};

export const TextInput = forwardRef<HTMLInputElement, Props>(
  function TextInput(
    {
      id,
      value,
      onChange,
      placeholder,
      disabled = false,
      autoFocus,
      className,
      onKeyDown,
      onFocus,
      onBlur,
      appearance = "form",
      size = "medium",
      source = "default",
    },
    ref
  ) {
    const generatedId = useId();
    const inputId = id ?? generatedId;

    return (
      <InputBase
        id={inputId}
        ref={ref}
        type="text"
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        autoFocus={autoFocus}
        onKeyDown={onKeyDown}
        onFocus={onFocus}
        onBlur={onBlur}
        className={clsx(
          "app-text-input",
          "input-control",
          `input-control--${appearance}`,
          `input-control--${size}`,
          source !== "default" && `input-control--${source}`,
          disabled && "input-control--disabled",
          className
        )}
      />
    );
  }
);



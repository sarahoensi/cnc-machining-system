import { forwardRef, useId } from "react";
import clsx from "clsx";
import "./TextInput.css";

type Props = {
  id?: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  autoFocus?: boolean;
  className?: string;
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
    },
    ref
  ) {
    const generatedId = useId();
    const inputId = id ?? generatedId;

    return (
      <input
        id={inputId}
        ref={ref}
        type="text"
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        autoFocus={autoFocus}
        className={clsx("app-text-input", className)}
      />
    );
  }
);

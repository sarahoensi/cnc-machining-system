import { KeyboardEventHandler, forwardRef, useId } from "react";
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
  onKeyDown?: KeyboardEventHandler<HTMLInputElement>;
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
        onKeyDown={onKeyDown}
        className={clsx("app-text-input", className)}
      />
    );
  }
);

// src/shared/ui/primitives/input/InputBase/InputBase.tsx

import { forwardRef, InputHTMLAttributes, ReactNode } from "react";
import clsx from "clsx";

type Props = Omit<InputHTMLAttributes<HTMLInputElement>, "size"> & {
  wrapperClassName?: string;
  inputClassName?: string;
  leftSlot?: ReactNode;
  rightSlot?: ReactNode;
};

export const InputBase = forwardRef<HTMLInputElement, Props>(function InputBase(
  { wrapperClassName, inputClassName, leftSlot, rightSlot, className, ...inputProps },
  ref,
) {
  const input = (
    <input ref={ref} className={clsx(inputClassName, className)} {...inputProps} />
  );

  if (!wrapperClassName && !leftSlot && !rightSlot) {
    return input;
  }

  return (
    <div className={wrapperClassName}>
      {leftSlot}
      {input}
      {rightSlot}
    </div>
  );
});

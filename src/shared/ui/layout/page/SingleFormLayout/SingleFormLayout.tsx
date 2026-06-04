import clsx from "clsx";
import type { ReactNode } from "react";
import {
  formWidthClassName,
  type FormWidth,
} from "../formWidth";
import "../formWidth.css";
import "./SingleFormLayout.css";

type Props = {
  form: ReactNode;
  formWidth?: FormWidth;
  className?: string;
};

export function SingleFormLayout({
  form,
  formWidth = "sm",
  className,
}: Props) {
  return (
    <div
      className={clsx(
        "single-form-layout",
        formWidthClassName(formWidth),
        className,
      )}
    >
      <div className="single-form-panel">{form}</div>
    </div>
  );
}

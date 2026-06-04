// shared/ui/layout/FormFigureLayout/FormFigureLayout.tsx

import "./FormFigureLayout.css";
import clsx from "clsx";
import {
  formWidthClassName,
  type FormWidth,
} from "../formWidth";
import "../formWidth.css";

type Props = {
  form: React.ReactNode;
  figure: React.ReactNode;
  formWidth?: FormWidth;
  className?: string;
};

export function FormFigureLayout({
  form,
  figure,
  formWidth = "sm",
  className,
}: Props) {
  return (
    <div
      className={clsx(
        "form-figure-layout",
        formWidthClassName(formWidth),
        className,
      )}
    >
      <div className="form-panel">{form}</div>
      <div className="figure-panel">{figure}</div>
    </div>
  );
}

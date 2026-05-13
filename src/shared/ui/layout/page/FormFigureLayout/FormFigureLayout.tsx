// shared/ui/layout/FormFigureLayout/FormFigureLayout.tsx

import "./FormFigureLayout.css";
import clsx from "clsx";

type Props = {
  form: React.ReactNode;
  figure: React.ReactNode;
  className?: string;
};

export function FormFigureLayout({ form, figure, className }: Props) {
  return (
    <div className={clsx("form-figure-layout", className)}>
      <div className="form-panel">{form}</div>
      <div className="figure-panel">{figure}</div>
    </div>
  );
}

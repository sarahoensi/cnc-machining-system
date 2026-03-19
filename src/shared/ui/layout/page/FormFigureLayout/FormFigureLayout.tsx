// shared/ui/layout/FormFigureLayout/FormFigureLayout.tsx

import "./FormFigureLayout.css";

type Props = {
  form: React.ReactNode;
  figure: React.ReactNode;
};

export function FormFigureLayout({ form, figure }: Props) {
  return (
    <div className="form-figure-layout">
      <div className="form-panel">{form}</div>
      <div className="figure-panel">{figure}</div>
    </div>
  );
}
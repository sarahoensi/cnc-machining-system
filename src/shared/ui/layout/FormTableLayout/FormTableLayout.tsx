// shared/ui/layout/FormTableLayout/FormTableLayout.tsx

import "./FormTableLayout.css";

type Props = {
  form: React.ReactNode;
  table: React.ReactNode;
};

export function FormTableLayout({ form, table }: Props) {
  return (
    <div className="form-table-layout">
      <div className="form-panel">{form}</div>
      <div className="table-panel">{table}</div>
    </div>
  );
}
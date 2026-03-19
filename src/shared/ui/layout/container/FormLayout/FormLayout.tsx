// shared/ui/layout/container/FormLayout/FormLayout.tsx

import "./FormLayout.css";

export function FormLayout({
  fields,
  error,
  actions,
}: {
  fields: React.ReactNode;
  error?: React.ReactNode;
  actions: React.ReactNode;
}) {
  return (
    <div className="form-layout">
      <div className="form-fields">{fields}</div>

      {error && (
        <div className="form-error-block">
          {error}
        </div>
      )}

      <div className="form-actions">
        {actions}
      </div>
    </div>
  );
}
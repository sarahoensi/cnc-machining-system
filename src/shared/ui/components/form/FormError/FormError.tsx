//

import "./FormError.css";

export function FormError({
  error,
}: {
  error?: string | string[];
}) {
  if (!error) return null;

  if (Array.isArray(error)) {
    return (
      <div className="form-error">
        <ul>
          {error.map((e, i) => (
            <li key={i}>{e}</li>
          ))}
        </ul>
      </div>
    );
  }

  return <div className="form-error">{error}</div>;
}
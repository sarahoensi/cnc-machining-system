//

export function FormError({ error }: { error?: string }) {
  if (!error) return null;

  return (
    <div className="form-error">
      {error}
    </div>
  );
}
// app/shell/Page.tsx

export function Page({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <h1 style={{ fontSize: "var(--font-xl)" }}>{title}</h1>
      <div style={{ marginTop: "var(--space-4)" }}>{children}</div>
    </div>
  );
}

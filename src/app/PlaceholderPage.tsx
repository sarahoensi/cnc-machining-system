type Props = {
  title: string;
};

export function PlaceholderPage({ title }: Props) {
  return (
    <div style={{ padding: 24 }}>
      <h1>{title}</h1>
      <p>Coming soon...</p>
    </div>
  );
}

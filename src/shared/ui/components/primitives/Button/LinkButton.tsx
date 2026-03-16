// shared/ui/components/primitives/Button/LinkButton.tsx

import "./LinkButton.css"

type Props = {
  onClick(): void;
  children: React.ReactNode;
};

export function LinkButton({ onClick, children }: Props) {
  return (
    <button
      type="button"
      className="link-button"
      onClick={onClick}
    >
      {children}
    </button>
  );
}
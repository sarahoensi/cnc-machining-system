// shared/ui/layout/StackedLayout/StackedLayout.tsx

import "./StackedLayout.css";

type Props = {
  header?: React.ReactNode;
  content: React.ReactNode;
  footer?: React.ReactNode;

  className?: string;
};

export function StackedLayout({
  header,
  content,
  footer,
  className,
}: Props) {
  return (
    <div className={`stacked-layout ${className ?? ""}`}>
      {header && (
        <div className="sl-header">
          {header}
        </div>
      )}

      <div className="sl-content">
        {content}
      </div>

      {footer && (
        <div className="sl-footer">
          {footer}
        </div>
      )}
    </div>
  );
}
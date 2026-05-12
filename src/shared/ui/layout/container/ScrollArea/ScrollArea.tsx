import clsx from "clsx";
import "./ScrollArea.css";

type Props = {
  children: React.ReactNode;
  className?: string;
};

export function ScrollArea({ children, className }: Props) {
  return (
    <div className={clsx("scroll-area", className)}>
      {children}
    </div>
  );
}
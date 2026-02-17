import type { ReactNode } from "react";

type Props = {
  title?: string;
  rightSlot?: ReactNode;
};

export function Topbar({
  title = "CNC System",
  rightSlot,
}: Props) {
  return (
    <header className="topbar">
      <div className="topbar-left">
        <h1 className="topbar-title">
          {title}
        </h1>
      </div>

      <div className="topbar-right">
        {rightSlot}
      </div>
    </header>
  );
}

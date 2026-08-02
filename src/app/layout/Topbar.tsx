// src/app/layout/Topbar.tsx

import { Button } from "@shared/ui/primitives/Button/Button";
import SettingsIcon from "@assets/settings-icon.svg";
import { SettingsMenu } from "../settings/SettingsMenu";
import "./Topbar.css";
import { useRef, useState } from "react";
import { useTitle } from "../providers/TitleContextProvider";

interface Props {
  toggleSidebar: () => void;
}

export function Topbar({ toggleSidebar }: Props) {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);

  const { title } = useTitle();

  return (
    <div className="topbar">
      <div className="topbar-left">
        <button className="hamburger" onClick={toggleSidebar}>
          ☰
        </button>
      </div>

      <h1 className="topbar-title">{title || "Wingardium Tooliosa"}</h1>

      <div className="topbar-right">
        <Button
          ref={buttonRef}
          variant="icon"
          size="icon"
          onClick={() => setIsSettingsOpen((prev) => !prev)}
        >
          <img src={SettingsIcon} alt="settings" className="icon-img" />
        </Button>

        {isSettingsOpen && (
          <SettingsMenu
            onClose={() => setIsSettingsOpen(false)}
            triggerRef={buttonRef}
          />
        )}
      </div>
    </div>
  );
}

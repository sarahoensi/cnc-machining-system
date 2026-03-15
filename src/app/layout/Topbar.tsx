// src/app/layout/Topbar.tsx

import { SettingsButton } from "@shared/ui/components/primitives/Button/Button";
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

        <h1 className="topbar-title">
        {title || "Wingardium Tooliosa"}
      </h1>

      <div className="topbar-right">
        <SettingsButton
          ref={buttonRef}
          onClick={() => setIsSettingsOpen(prev => !prev)}
        />


        {isSettingsOpen && (
          <SettingsMenu onClose={() => setIsSettingsOpen(false)}
            triggerRef={buttonRef} />
        )}
      </div>
    </div>
  );
}

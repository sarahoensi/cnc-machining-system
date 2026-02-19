// src/app/layout/Topbar.tsx

import { SettingsMenu } from "../settings/SettingsMenu";
import "./Topbar.css";
import { useRef, useState } from "react";


interface Props {
  toggleSidebar: () => void;
}

export function Topbar({ toggleSidebar }: Props) {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);


  return (
    <div className="topbar">
      <div className="topbar-left">
        <button className="hamburger" onClick={toggleSidebar}>
          ☰
        </button>
      </div>

      <div className="topbar-center">
        <h1 className="topbar-title">My App</h1>
      </div>

      <div className="topbar-right">
        <button
          ref={buttonRef}
          className="settings-button"
          onClick={() => setIsSettingsOpen(prev => !prev)}
        >
          ⚙
        </button>

        {isSettingsOpen && (
          <SettingsMenu onClose={() => setIsSettingsOpen(false)} 
          triggerRef={buttonRef}/>
        )}
      </div>
    </div>
  );
}

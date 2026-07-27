// src/app/settings/SettingsMenu.tsx

import { useEffect, useRef, useState, type ComponentType } from "react";
import "./settings.css";

import { ThemeSettings } from "./panels/ThemeSettings";
import { DecimalSettings } from "./panels/DecimalSettings";

type SettingsItem = {
  key: string;
  label: string;
  component: ComponentType;
};

const SETTINGS_MENU: readonly SettingsItem[] = [
  { key: "theme", label: "🎨 Tema", component: ThemeSettings },
  { key: "decimals", label: "🔢 Desimaler", component: DecimalSettings },
] as const;

type Props = {
  onClose: () => void;
  triggerRef?: React.RefObject<HTMLElement | null>;
};

export function SettingsMenu({ onClose, triggerRef }: Props) {
  const ref = useRef<HTMLDivElement>(null);
  const [activeKey, setActiveKey] = useState<string | null>(null);

  /* --------------------------------------------
     Close on click outside (FIXED)
  -------------------------------------------- */
  useEffect(() => {
    function handlePointerDown(e: MouseEvent) {
      const target = e.target as Node;

      if (!ref.current) return;

      const clickedInsideMenu = ref.current.contains(target);
      const clickedTrigger = triggerRef?.current?.contains(target);

      if (!clickedInsideMenu && !clickedTrigger) {
        onClose();
      }
    }

    // IMPORTANT: use mousedown instead of click
    document.addEventListener("mousedown", handlePointerDown);

    return () => document.removeEventListener("mousedown", handlePointerDown);
  }, [onClose]);

  /* --------------------------------------------
     Close on Escape
  -------------------------------------------- */
  useEffect(() => {
    function handleEscape(e: KeyboardEvent) {
      if (e.key === "Escape") {
        onClose();
      }
    }

    document.addEventListener("keydown", handleEscape);

    return () => document.removeEventListener("keydown", handleEscape);
  }, [onClose]);

  const activeItem = SETTINGS_MENU.find((item) => item.key === activeKey);

  /* --------------------------------------------
     Render
  -------------------------------------------- */

  return (
    <div ref={ref} className="settings-menu">
      <div className="menu-content">
        <div className="menu-column">
          {SETTINGS_MENU.map((item) => (
            <div
              key={item.key}
              className="menu-item"
              onMouseEnter={() => setActiveKey(item.key)}
            >
              {item.label}
              <span className="chevron">›</span>
            </div>
          ))}
        </div>

        {activeItem && (
          <div className="submenu-flyout">
            <activeItem.component />
          </div>
        )}
      </div>
    </div>
  );
}

import {
  useEffect,
  useRef,
  useState,
  type ComponentType,
} from "react";
import "./SettingsMenu.css";

import { ThemeSettings } from "./theme/ThemeSettings";
import { DecimalSettings } from "./decimals/DecimalSettings";

/* ============================================================
   Menu Configuration (type-safe)
============================================================ */

type SettingsItem = {
  key: string;
  label: string;
  component: ComponentType;
};

const SETTINGS_MENU: readonly SettingsItem[] = [
  {
    key: "theme",
    label: "🎨 Tema",
    component: ThemeSettings,
  },
  {
    key: "decimals",
    label: "🔢 Desimaler",
    component: DecimalSettings,
  },
] as const;

/* ============================================================
   Props
============================================================ */

type Props = {
  onClose: () => void;
};

/* ============================================================
   Component
============================================================ */

export function SettingsMenu({ onClose }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [activeKey, setActiveKey] =
    useState<string | null>(null);

  /* --------------------------------------------
     Close on click outside
  -------------------------------------------- */
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (!containerRef.current) return;

      if (
        !containerRef.current.contains(
          e.target as Node
        )
      ) {
        onClose();
      }
    }

    document.addEventListener(
      "mousedown",
      handleClickOutside
    );

    return () => {
      document.removeEventListener(
        "mousedown",
        handleClickOutside
      );
    };
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

    document.addEventListener(
      "keydown",
      handleEscape
    );

    return () => {
      document.removeEventListener(
        "keydown",
        handleEscape
      );
    };
  }, [onClose]);

  /* --------------------------------------------
     Active item
  -------------------------------------------- */
  const activeItem = SETTINGS_MENU.find(
    (item) => item.key === activeKey
  );

  /* --------------------------------------------
     Render
  -------------------------------------------- */

  return (
    <div
      ref={containerRef}
      className="settings-menu"
      onMouseLeave={() => setActiveKey(null)}
    >
      {/* Left column */}
      <div className="settings-menu__list">
        {SETTINGS_MENU.map((item) => (
          <button
            key={item.key}
            type="button"
            className="settings-menu__item"
            onMouseEnter={() =>
              setActiveKey(item.key)
            }
          >
            <span>{item.label}</span>
            <span className="chevron">›</span>
          </button>
        ))}
      </div>

      {/* Flyout panel */}
      {activeItem && (
        <div className="settings-menu__flyout">
          <activeItem.component />
        </div>
      )}
    </div>
  );
}

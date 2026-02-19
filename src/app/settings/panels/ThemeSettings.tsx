// app/settings/theme/ThemeSettings.tsx

import { useTheme } from "@app/providers/ThemeProvider";
import "../settings.css";


export function ThemeSettings() {
  const { theme, setTheme } = useTheme();
  const themes = ["default", "pink", "forest", "dark"] as const;

  return (
    <div className="settings-panel">
      <h3 className="settings-title">Tema</h3>

      <div className="settings-button-group">
        {themes.map(t => (
          <button
            key={t}
            className={`settings-button ${theme === t ? "active" : ""}`}
            onClick={() => setTheme(t)}
          >
            {t}
          </button>
        ))}
      </div>
    </div>
  );
}

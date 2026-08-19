// app/settings/apprentice/ApprenticeSettings.tsx

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import "../settings.css";

export function ApprenticeSettings() {
  const { apprenticeMode, setApprenticeMode } = useDisplaySettings();

  return (
    <div className="settings-panel">
      <h3 className="settings-title">Lærlingmodus</h3>

      <div className="settings-button-group">
        <button
          className={`settings-button ${apprenticeMode ? "active" : ""}`}
          onClick={() => setApprenticeMode(true)}
        >
          På
        </button>
        <button
          className={`settings-button ${!apprenticeMode ? "active" : ""}`}
          onClick={() => setApprenticeMode(false)}
        >
          Av
        </button>
      </div>
    </div>
  );
}

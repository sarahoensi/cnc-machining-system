// app/settings/decimals/DecimalSetting.tsx

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import "../settings.css";


export function DecimalSettings() {
  const { decimals, setDecimals } = useDisplaySettings();
  const options = [0, 1, 2, 3, 4, 5, 6] as const;

  return (
    <div className="settings-panel">
      <h3 className="settings-title">Desimaler</h3>

      <div className="settings-button-group">
        {options.map(value => (
          <button
            key={value}
            className={`settings-button ${decimals === value ? "active" : ""}`}
            onClick={() => setDecimals(value)}
          >
            {value}
          </button>
        ))}
      </div>
    </div>
  );
}

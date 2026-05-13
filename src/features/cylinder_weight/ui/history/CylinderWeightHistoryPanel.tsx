import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { ScrollArea } from "@shared/ui/layout/container/ScrollArea/ScrollArea";
import { HistoryCard } from "@shared/ui/layout/container/HistoryCard/HistoryCard";
import { Button } from "@shared/ui/primitives/Button/Button";
import "./CylinderWeightHistoryPanel.css";

type CylinderWeightSavedResult = {
  id: string;
  materialName: string;
  density: number;
  outerDiameter: number;
  innerDiameter: number;
  length: number;
  mass: number;
  units: {
    density: "kg/m³";
    outerDiameter: "mm";
    innerDiameter: "mm";
    length: "mm";
    mass: "kg";
  };
};

type Props = {
  history: CylinderWeightSavedResult[];
  onLoad(entry: CylinderWeightSavedResult): void;
  onDelete(id: string): void;
  onClear(): void;
};

export function CylinderWeightHistoryPanel({
  history,
  onLoad,
  onDelete,
  onClear,
}: Props) {
  const { decimals } = useDisplaySettings();

  return (
    <section className="cylinder-weight-history">
      <h3 className="cylinder-weight-history-title">Saved results</h3>

      <ScrollArea className="cylinder-weight-history-scroll">
        {history.length === 0 && (
          <div className="cylinder-weight-history-empty">
            No saved results yet
          </div>
        )}

        {history.length > 0 && (
          <div className="cylinder-weight-history-list">
            {history.map((entry) => (
              <HistoryCard
                key={entry.id}
                items={buildItems(entry, decimals)}
                columns={2}
                onClick={() => onLoad(entry)}
                onDelete={() => onDelete(entry.id)}
              />
            ))}
          </div>
        )}
      </ScrollArea>

      {history.length > 0 && (
        <div className="cylinder-weight-history-actions">
          <Button variant="secondary" size="small" onClick={onClear}>
            Clear all results
          </Button>
        </div>
      )}
    </section>
  );
}

function buildItems(entry: CylinderWeightSavedResult, decimals: number) {
  return [
    {
      label: "Material",
      value: entry.materialName,
    },
    {
      label: "Density",
      value: formatNumber(entry.density, decimals),
      unit: entry.units.density,
    },
    {
      label: "OD",
      value: formatNumber(entry.outerDiameter, decimals),
      unit: entry.units.outerDiameter,
    },
    {
      label: "ID",
      value: formatNumber(entry.innerDiameter, decimals),
      unit: entry.units.innerDiameter,
    },
    {
      label: "Length",
      value: formatNumber(entry.length, decimals),
      unit: entry.units.length,
    },
    {
      label: "Mass",
      value: formatNumber(entry.mass, decimals),
      unit: entry.units.mass,
    },
  ];
}

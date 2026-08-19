import type { ThreadKey } from "../domain/threadForm";
import { threadTooltips } from "./threadTooltips";

type ThreadFieldConfig = {
  key: ThreadKey;
  label: string;
  shortLabel?: string;
  tooltip?: string;
  unit?: string;
  readOnly?: boolean;
};

export const threadResultFieldConfig: ThreadFieldConfig[] = [
  {
    key: "drill_diameter",
    label: "Tap drill",
    shortLabel: "Tap drill",
    unit: "mm",
    tooltip: threadTooltips.drill_diameter,
    readOnly: true,
  },
  {
    key: "thread_depth",
    label: "Radial thread depth",
    shortLabel: "Depth",
    unit: "mm",
    tooltip: threadTooltips.thread_depth,
    readOnly: true,
  },
];

export function buildThreadResultFieldConfig(drillDiameterLabel = "Tap drill") {
  return threadResultFieldConfig.map((config) =>
    config.key === "drill_diameter"
      ? { ...config, label: drillDiameterLabel, shortLabel: drillDiameterLabel }
      : config,
  );
}

export const threadHistoryFieldConfig: ThreadFieldConfig[] = [
  {
    key: "drill_diameter",
    label: "Tap drill",
    shortLabel: "Tap drill",
    unit: "mm",
  },
  {
    key: "thread_depth",
    label: "Radial thread depth",
    shortLabel: "Depth",
    unit: "mm",
  },
];

export const threadSelectConfig = {
  type: {
    label: "Thread type",
    tooltip: threadTooltips.type,
  },
  size: {
    label: "Thread size",
    tooltip: threadTooltips.size,
  },
  pitch: {
    label: "Pitch",
    tooltip: threadTooltips.pitch,
  },
};

export type ThreadApiType = "metric" | "unc" | "unf" | "bsp";
export type ThreadType = "metric" | "unified" | "bsp";

export type ThreadSizeOption = {
  value: string;
  label: string;
  majorDiameterMm: number;
  pitches: ThreadPitchOption[];
};

export type ThreadPitchOption = {
  value: string;
  label: string;
  pitchMm: number;
  series: string;
  isDefaultPitch: boolean;
  sourceType?: ThreadApiType;
};

export type ThreadTypeOption = {
  value: ThreadApiType;
  label: string;
};

export type ThreadOptionsResponse = {
  types: ThreadTypeOption[];
  metric: ThreadSizeOption[];
  unc: ThreadSizeOption[];
  unf: ThreadSizeOption[];
  bsp: ThreadSizeOption[];
};

export type ThreadCalculationInput = {
  type: ThreadApiType;
  size: string;
  pitch: string;
};

export type ThreadCalculationResult = {
  drillDiameterMm: number;
  threadDepthMm: number;
};

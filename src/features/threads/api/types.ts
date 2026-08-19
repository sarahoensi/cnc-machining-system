export type ThreadApiType = "metric" | "unc" | "unf" | "bsp" | "npt";
export type ThreadType = "metric" | "unified" | "bsp" | "npt";

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
  tapDrillBasis?: string;
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
  npt: ThreadSizeOption[];
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

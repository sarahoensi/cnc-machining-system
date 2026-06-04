// features/tolerances/api/types.ts

export type ToleranceObjectType = "hole" | "shaft";
export type ToleranceMode = ToleranceObjectType;

export type ToleranceOption = {
  feature: ToleranceObjectType;
  zone: string;
  grades: number[];
};

export type ToleranceOptionsResponse = {
  holes: ToleranceOption[];
  shafts: ToleranceOption[];
};

export type LookupIso286ToleranceRequest = {
  feature: ToleranceObjectType;
  nominalMm: number;
  code: string;
};

export type Iso286MemberResult = {
  code: string;
  zone: string;
  grade: number;
  upper_um: number;
  lower_um: number;
  max_mm: number;
  min_mm: number;
  source_table: string | null;
  source_file: string | null;
};

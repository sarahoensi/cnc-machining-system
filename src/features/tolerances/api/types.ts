export type ToleranceObjectType = "hole" | "shaft";

export type CalculateIso286FitRequest = {
  nominal_mm: number;
  hole: string;
  shaft: string;
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

export type Iso286FitSummary = {
  min_clearance_mm: number;
  max_clearance_mm: number;
  type: "clearance" | "interference" | "transition";
};

export type Iso286FitResult = {
  nominal_mm: number;
  hole: Iso286MemberResult;
  shaft: Iso286MemberResult;
  fit: Iso286FitSummary;
};

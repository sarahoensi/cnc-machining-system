// features/right_triangle/api/types.ts

export type SolveRightTriangleRequest =
  | { type: "Legs"; a_mm: number; b_mm: number }
  | { type: "LegAAndHypotenuse"; a_mm: number; c_mm: number }
  | { type: "LegBAndHypotenuse"; b_mm: number; c_mm: number }
  | { type: "LegAAndAlpha"; a_mm: number; alpha_deg: number }
  | { type: "LegAAndBeta"; a_mm: number; beta_deg: number }
  | { type: "LegBAndAlpha"; b_mm: number; alpha_deg: number }
  | { type: "LegBAndBeta"; b_mm: number; beta_deg: number }
  | { type: "HypotenuseAndAlpha"; c_mm: number; alpha_deg: number }
  | { type: "HypotenuseAndBeta"; c_mm: number; beta_deg: number };

export type SolveRightTriangleResponse = {
  a_mm: number;
  b_mm: number;
  c_mm: number;
  alpha_deg: number;
  beta_deg: number;
};
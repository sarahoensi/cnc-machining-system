// features/helix/api/types.ts

export type HelixMode = "Inner" | "Outer";

export type SolveHelixRequest =
  | {
      type: "Pitch";
      mode: HelixMode;
      diameter_mm: number;
      tool_diameter_mm: number;
      pitch_mm_per_rev: number;
    }
  | {
      type: "Angle";
      mode: HelixMode;
      diameter_mm: number;
      tool_diameter_mm: number;
      angle_deg: number;
    };

export type SolveHelixResponse = {
  pitch_mm_per_rev: number;
  angle_deg: number;
};
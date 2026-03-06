// features/helix/api/types.ts

export type HelixMode = "Inner" | "Outer";

export type SolveHelixRequest =
  | {
      type: "Pitch";
      mode: HelixMode;
      diameter: number;
      tool_diameter: number;
      pitch: number;
    }
  | {
      type: "Angle";
      mode: HelixMode;
      diameter: number;
      tool_diameter: number;
      angle: number;
    };

export type SolveHelixResponse = {
  pitch: number;
  angle: number;
};
// features/finishing/api/types.ts

export type FinishingMode = "Inner" | "Outer";

/* ============================================================
   Generate plan
============================================================ */

export type GenerateFinishingPlanRequest =
  | {
      type: "ByCuts";
      mode: FinishingMode;
      start_diameter_mm: number;
      target_diameter_mm: number;
      cuts: number;
    }
  | {
      type: "ByRadialEngagement";
      mode: FinishingMode;
      start_diameter_mm: number;
      target_diameter_mm: number;
      radial_engagement_mm: number;
    };

/* ============================================================
   Register measurement
============================================================ */

export type RegisterFinishingMeasurementRequest = {
  step_number: number;
  measurement_mm: number;
};

/* ============================================================
   Response
============================================================ */

export type FinishingStep = {
  index: number;
  startMm: number;
  plannedDeltaMm: number;
  plannedEndMm: number;
  measurementMm: number | null;
};

export type FinishingExecutionResponse = {
  activeStep: number | null;
  finished: boolean;
  steps: FinishingStep[];
};

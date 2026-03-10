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
  execution_id: string;
  step_number: number;
  measurement_mm: number;
};

/* ============================================================
   Response
============================================================ */

export type FinishingStep = {
  index: number;
  start_mm: number;
  planned_delta_mm: number;
  planned_end_mm: number;
  measurement_mm: number | null;
};

export type FinishingExecutionResponse = {
  execution_id: string;

    /** step currently waiting for measurement */
  active_step: number | null;

  /** execution completed */
  finished: boolean;

  steps: FinishingStep[];
};
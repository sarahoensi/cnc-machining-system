import type { FieldState } from "@shared/form/types/fields";

import type { CuttingDataKey } from "./cuttingDataForm";

export function validateCuttingDataForm(
  fields: Record<CuttingDataKey, FieldState>,
): string[] | null {
  const errors: string[] = [];

  const diameter = hasValue(fields.diameter);
  const rpm = hasValue(fields.rpm);
  const vc = hasValue(fields.cutting_speed);
  const teeth = hasValue(fields.teeth);
  const chip = hasValue(fields.chip_load);
  const feed = hasValue(fields.feed_rate);

  const canSolveRotation = diameter && (rpm || vc);
  const canResolveRpm = rpm || vc;
  const wantsFeedCalculation = teeth || chip || feed;
  const canSolveFeed = diameter && teeth && canResolveRpm && (chip || feed);

  if (canSolveRotation || canSolveFeed) {
    return null;
  }

  if (!diameter && !rpm && !vc && !wantsFeedCalculation) {
    return [
      "Fill in D and either Vc or n.",
      "For feed, fill in z and either F or fz.",
    ];
  }

  if (!diameter) {
    errors.push("Tool diameter D must be filled in.");
  }

  if (!rpm && !vc) {
    errors.push("Fill in either cutting speed Vc or rotations n.");
  }

  if (wantsFeedCalculation && !teeth) {
    errors.push("To calculate feed, toothcount z must be filled in.");
  }

  if (wantsFeedCalculation && !feed && !chip) {
    errors.push("To calculate feed, fill in either feed rate F or chip load fz.");
  }

  if (wantsFeedCalculation && !canResolveRpm) {
    errors.push("To calculate feed, fill in rotations n or cutting speed Vc.");
  }

  return errors.length > 0 ? errors : null;
}

function hasValue(field: FieldState) {
  return field.value.trim().length > 0;
}

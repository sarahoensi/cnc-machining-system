// shared/types/execution.ts

import type { ProcessStatus } from "./common";
import type { FieldState } from "./fields";

export type ExecutionStep = {
  index: number;

  startDiameter: number;
  deltaD: number;

  measurement: FieldState;

  status: "pending" | "active" | "completed";
};

export type ExecutionState = {
  status: ProcessStatus;
  steps: ExecutionStep[];
  activeIndex: number;
};

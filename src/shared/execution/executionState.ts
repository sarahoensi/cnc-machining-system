// shared/execution/executionState.ts


export type ExecutionValue = {
  value: string;
};

export type ExecutionStepStatus =
  | "pending"
  | "active"
  | "completed";

export type ExecutionStep<T> = {
  index: number;
  data: T;
  measurement: ExecutionValue;
  status: ExecutionStepStatus;
  editable: boolean;
};

export type ExecutionState<T> = {
  steps: ExecutionStep<T>[];
  activeIndex: number;
};

/* ============================================================
   Build execution state
============================================================ */

export function createExecutionState<T>(steps: {
  index: number;
  data: T;
  measurement?: number | null;
}[]): ExecutionState<T> {

  const firstIncomplete =
    steps.findIndex(s => s.measurement == null);

  const activeIndex =
    firstIncomplete === -1
      ? steps.length
      : firstIncomplete;

  const lastMeasuredIndex =
    steps
      .map((s, i) => s.measurement != null ? i : -1)
      .filter(i => i !== -1)
      .pop();

  const mappedSteps: ExecutionStep<T>[] =
    steps.map((step, i) => {

      const status: ExecutionStepStatus =
        step.measurement != null
          ? "completed"
          : i === activeIndex
          ? "active"
          : "pending";

      const editable =
        status === "active" ||
        i === lastMeasuredIndex;

      const measurement: ExecutionValue = {
        value:
          step.measurement == null
            ? ""
            : step.measurement.toString(),
      };

      return {
        index: step.index,
        data: step.data,
        measurement,
        status,
        editable,
      };
    });

  return {
    steps: mappedSteps,
    activeIndex,
  };
}

/* ============================================================
   Helpers
============================================================ */

export function isStepActive(step: ExecutionStep<any>) {
  return step.status === "active";
}

export function isStepCompleted(step: ExecutionStep<any>) {
  return step.status === "completed";
}

export function isStepPending(step: ExecutionStep<any>) {
  return step.status === "pending";
}
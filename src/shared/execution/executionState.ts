// shared/execution/executionState.ts

export type ExecutionValue = {
  value: string;
};

export type ExecutionStepStatus = "pending" | "active" | "completed";

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
  finished: boolean;
};

/* ============================================================
   Build execution state
============================================================ */

export function createExecutionState<T>(
  steps: {
    index: number;
    data: T;
    measurement?: number | null;
  }[],
  finished: boolean,
): ExecutionState<T> {
  const firstIncomplete = steps.findIndex((s) => s.measurement == null);

  const activeIndex = firstIncomplete === -1 ? steps.length : firstIncomplete;

  const lastMeasuredIndex = steps
    .map((s, i) => (s.measurement != null ? i : -1))
    .filter((i) => i !== -1)
    .pop();

  const mappedSteps: ExecutionStep<T>[] = steps.map((step, i) => {
    const status: ExecutionStepStatus =
      step.measurement != null ? "completed" : i === activeIndex ? "active" : "pending";

    const editable = status === "active" || i === lastMeasuredIndex;

    const measurement: ExecutionValue = {
      value: step.measurement == null ? "" : step.measurement.toString(),
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
    finished,
  };
}

/* ============================================================
   Helpers
============================================================ */

export function isStepActive<T>(step: ExecutionStep<T>) {
  return step.status === "active";
}

export function isStepCompleted<T>(step: ExecutionStep<T>) {
  return step.status === "completed";
}

export function isStepPending<T>(step: ExecutionStep<T>) {
  return step.status === "pending";
}

/* ============================================================
   Derived helpers (UI-safe, but domain-driven)
============================================================ */

export function isStepEditableCompleted<T>(step: ExecutionStep<T>, finished: boolean) {
  return !finished && step.status === "completed" && step.editable;
}

export function isStepInputEditable<T>(
  step: ExecutionStep<T>,
  finished: boolean,
  isEditing: boolean,
) {
  if (isEditing) return true;

  return !finished && step.status === "active";
}

export function getStepMeasurementValue<T>(step: ExecutionStep<T>) {
  return step.measurement.value ?? "";
}

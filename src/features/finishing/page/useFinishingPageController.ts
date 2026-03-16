// features/finishing/pagee/useFinishingPageController.ts

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { handleGenerateAsync } from "@shared/form";

import { createInitialFinishingForm } from "../plan/domain/finishingForm";
import { parseFinishingPlan } from "../plan/domain/parseFinishingPlan";

import { buildRegisterRequest } from "../execution/domain/buildRegisterRequest";

import { generateFinishingPlan } from "../plan/api/generateFinishingPlan";
import { registerFinishingMeasurement } from "../execution/api/registerFinishingMeasurement";

import { mapFinishingExecution } from "../execution/domain/mapExecution";

import type { ExecutionState } from "@shared/execution";
import type { FinishingStepData } from "../execution/domain/mapExecution";


export function useFinishingPageController() {

  const [form, setForm] = useFeatureForm(
    "finishing",
    createInitialFinishingForm
  );

  const [execution, setExecution] = useFeatureForm<
    ExecutionState<FinishingStepData> | null
  >(
    "finishing-execution",
    () => null
  );

  const formReadOnly = execution !== null;

  /* ============================================================
     Helpers
  ============================================================ */

  function executionHasMeasurements() {

    if (!execution) return false;

    return execution.steps.some(
      s => s.measurement.value !== ""
    );
  }

  function confirmExecutionReset(): boolean {

    if (!executionHasMeasurements()) {
      return true;
    }

    return window.confirm(
      "Det finnes registrerte målinger.\n\n" +
      "Hvis du endrer planen vil utførelsen bli slettet.\n\n" +
      "Vil du fortsette?"
    );
  }

  function clearExecution() {
    setExecution(null);
  }

  /* ============================================================
     Form update
  ============================================================ */

  function updateForm(nextForm: any) {

    if (execution && !confirmExecutionReset()) {
      return;
    }

    clearExecution();
    setForm(nextForm);
  }

  /* ============================================================
     Generate plan
  ============================================================ */

  async function generate() {

    const { form: nextForm, execution: result } =
      await handleGenerateAsync(
        form,
        parseFinishingPlan,
        generateFinishingPlan
      );

    setForm(nextForm);

    if (result) {

      setExecution(
        mapFinishingExecution(result)
      );
    }
  }

  /* ============================================================
     Register measurement
  ============================================================ */

  async function registerMeasurement(
    step: number,
    measurement: number
  ) {

    const request = buildRegisterRequest(
      step,
      measurement
    );

    const result =
      await registerFinishingMeasurement(request);

    setExecution(
      mapFinishingExecution(result)
    );
  }

  /* ============================================================
     Reset
  ============================================================ */

  function reset() {

    if (execution && !confirmExecutionReset()) {
      return;
    }

    clearExecution();
    setForm(createInitialFinishingForm());
  }

  function editPlan() {

    if (!confirmExecutionReset()) return;

    clearExecution();
  }

  return {

    form,
    execution,
    formReadOnly,

    updateForm,
    generate,
    registerMeasurement,
    reset,
    editPlan,

  };
}
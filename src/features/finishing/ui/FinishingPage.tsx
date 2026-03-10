// features/finishing/ui/FinishingPage.tsx

import { useState } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { handleGenerateAsync } from "@shared/form";

import {
  createInitialFinishingForm,
} from "../domain/plan/finishingForm";

import { parseFinishingPlan } from "../domain/plan/parseFinishingPlan";

import { buildRegisterRequest } from "../domain/execution/buildRegisterRequest";

import { generateFinishingPlan } from "../api/plan/generateFinishingPlan";
import { registerFinishingMeasurement } from "../api/execution/registerFinishingMeasurement";

import { mapFinishingExecution } from "../domain/execution/mapExecution";

import type { ExecutionState } from "@shared/execution";
import type { FinishingStepData } from "../domain/execution/mapExecution";

import { PlanForm } from "./plan/PlanForm";
import { FinishingExecutionTable } from "./execution/ExecutionTable";

export function FinishingPage() {

  const [form, setForm] = useFeatureForm(
    "finishing",
    createInitialFinishingForm
  );

  const [executionId, setExecutionId] =
    useState<string | null>(null);

  const [execution, setExecution] =
    useState<ExecutionState<FinishingStepData> | null>(null);

  /* ============================================================
     Generate plan
  ============================================================ */

  async function onGenerate() {

    const { form: nextForm, execution: result } =
      await handleGenerateAsync(
        form,
        parseFinishingPlan,
        generateFinishingPlan
      );

    setForm(nextForm);

    if (result) {

      setExecutionId(result.execution_id);

      setExecution(
        mapFinishingExecution(result)
      );
    }
  }

  /* ============================================================
     Register measurement
  ============================================================ */

  async function onRegisterMeasurement(
    step: number,
    measurement: number
  ) {

    if (!executionId) return;

    const request = buildRegisterRequest(
      executionId,
      step,
      measurement
    );

    const result =
      await registerFinishingMeasurement(request);

    /* IMPORTANT:
       Replace execution with backend result
       (never mutate locally)
    */

    setExecution(
      mapFinishingExecution(result)
    );
  }

  /* ============================================================
     Reset
  ============================================================ */

  function onReset() {

    setForm(createInitialFinishingForm());

    setExecution(null);
    setExecutionId(null);
  }

  /* ============================================================
     Render
  ============================================================ */

  return (

    <div className="app-content split">

      <div className="app-left">

        <PlanForm
          form={form}
          setForm={setForm}
          onGenerate={onGenerate}
          onReset={onReset}
        />

      </div>

      <div className="app-right">

        {execution ? (

          <FinishingExecutionTable
            execution={execution}
            onRegisterMeasurement={onRegisterMeasurement}
          />

        ) : (

          <p className="hint">
            Ingen utførelse startet ennå
          </p>

        )}

      </div>

    </div>
  );
}
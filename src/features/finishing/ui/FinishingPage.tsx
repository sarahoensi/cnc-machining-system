// features/finishing/ui/FinishingPage.tsx

//import { useState } from "react";

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

import { Page } from "@app/shell/Page";


export function FinishingPage() {

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
       Form update wrapper
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

    async function onGenerate() {

        const { form: nextForm, execution: result } =
            await handleGenerateAsync(
                form,
                parseFinishingPlan,
                generateFinishingPlan
            );

        setForm(nextForm);

        if (result) {

            console.log("Finishing response:", result);

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

    function onReset() {

        if (execution && !confirmExecutionReset()) {
            return;
        }

        clearExecution();

        setForm(createInitialFinishingForm());
    }

    /* ============================================================
       Render
    ============================================================ */

    return (
        <Page title="Finishing">

        <div className="app-content split">

            <div className="app-left">

                <PlanForm
                    form={form}
                    setForm={updateForm}
                    onGenerate={onGenerate}
                    onReset={onReset}
                    onEdit={() => {
                        if (!confirmExecutionReset()) return;
                        clearExecution();
                    }}
                    readOnly={formReadOnly}
                />

            </div>

            <div className="app-right">

                {execution ? (

                    <FinishingExecutionTable
                        execution={execution}
                        onRegisterMeasurement={
                            onRegisterMeasurement
                        }
                    />

                ) : (

                    <p className="hint">
                        Ingen utførelse startet ennå
                    </p>

                )}

            </div>

        </div>

        </Page>

    );
}
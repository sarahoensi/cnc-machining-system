// features/finishing/ui/FinishingPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";

import { FormFigureLayout } from "@shared/ui/layout/FormFigureLayout/FormFigureLayout";

import { PlanForm } from "../plan/ui/PlanForm";
import { ExecutionView } from "../execution/ui/ExecutionView/ExecutionView";

import { useFinishingPageController } from "../page/useFinishingPageController";

export function FinishingPage() {

  usePageTitle("Finishing");

  const finishing = useFinishingPageController();

  const formContent = (
    <PlanForm
      form={finishing.form}
      setForm={finishing.updateForm}
      onGenerate={finishing.generate}
      onReset={finishing.reset}
      onEdit={finishing.editPlan}
      readOnly={finishing.formReadOnly}
    />
  );

  const formSummary = (
    {
        mode: finishing.form.extras.mode,
        startDiameter:
          finishing.form.fields.start_diameter_mm.value,
        targetDiameter:
          finishing.form.fields.target_diameter_mm.value,
        cuts: finishing.form.fields.cuts.value,
        radialEngagement:
          finishing.form.fields.radial_engagement_mm.value,
      }
  );

  if (!finishing.execution) {
    return (
      <FormFigureLayout
        form={formContent}
        figure={null}
      />
    );
  }

  return (
    <ExecutionView
      execution={finishing.execution}
      summary={formSummary}
      onRegisterMeasurement={finishing.registerMeasurement}
      onEditPlan={finishing.editPlan}
      onReset={finishing.reset}
    />
  );
}
import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { Field } from "@shared/ui/form/Field";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { FormSelectMenuField } from "@shared/ui/form/fields/FormSelectMenuField";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/input";
import { SelectMenuLabel } from "@shared/ui/primitives/Select";
import type { ReactNode } from "react";

import type { useThreadsPageController } from "../useThreadsPageController";
import { threadResultFieldConfig, threadSelectConfig } from "../threadFieldConfig";
import "../ThreadsPage.css";

type Props = {
  controller: ReturnType<typeof useThreadsPageController>;
};

export function ThreadsForm({ controller }: Props) {
  const { form, navigation } = controller;

  const error = form.formError ? <FormError error={form.formError} /> : null;
  const selectedPitchOption = controller.pitchOptions.find(
    (option) => option.value === form.fields.pitch.value,
  );
  const selectedSizeOption = controller.sizeOptions.find(
    (option) => option.value === form.fields.size.value,
  );

  if (controller.apprentice.enabled) {
    return (
      <ThreadApprenticeForm
        controller={controller}
        error={error}
        selectedPitchOption={selectedPitchOption}
        selectedSizeOption={selectedSizeOption}
      />
    );
  }

  return (
    <div ref={navigation.containerRef} className="threads-form-root">
      <FormLayout
        error={error}
        actions={
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
            calculateRef={navigation.registerSubmitAction}
            onCalculateKeyDown={navigation.handleSubmitActionKeyDown}
          >
            <Button
              variant="secondary"
              size="medium"
              onClick={controller.save}
              disabled={form.status !== "solved"}
            >
              Save result
            </Button>
          </FormActions>
        }
        actionsPlacement="bottom"
      >
        <FormGrid areas={[["selection"], ["result"]]}>
          <FormGrid.Area name="selection">
            <ThreadSelectionFields
              controller={controller}
              selectedPitchOption={selectedPitchOption}
              selectedSizeOption={selectedSizeOption}
            />
          </FormGrid.Area>

          <FormGrid.Area name="result" className="threads-result-grid">
            <FormNumberFields
              configs={threadResultFieldConfig}
              fields={form.fields}
              onChange={controller.onFieldChange}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

function ThreadApprenticeForm({
  controller,
  error,
  selectedPitchOption,
  selectedSizeOption,
}: {
  controller: ReturnType<typeof useThreadsPageController>;
  error: ReactNode;
  selectedPitchOption: ReturnType<
    typeof useThreadsPageController
  >["pitchOptions"][number] | undefined;
  selectedSizeOption: ReturnType<
    typeof useThreadsPageController
  >["sizeOptions"][number] | undefined;
}) {
  const { form, navigation, apprentice } = controller;

  return (
    <div ref={navigation.containerRef} className="threads-form-root">
      <FormLayout
        error={error}
        actions={
          <div className="threads-apprentice-actions">
            <Button
              variant="primary"
              size="large"
              onClick={() => {
                apprentice.checkAnswer("drill");
                apprentice.checkAnswer("depth");
              }}
              disabled={!apprentice.guide}
            >
              Sjekk svar
            </Button>
            <Button
              variant="secondary"
              size="medium"
              onClick={controller.save}
              disabled={form.status !== "solved"}
            >
              Save result
            </Button>
            <Button variant="danger" size="medium" onClick={controller.resetForm}>
              Clear form
            </Button>
          </div>
        }
        actionsPlacement="bottom"
      >
        <FormGrid areas={[["selection"], ["work"]]}>
          <FormGrid.Area name="selection">
            <ThreadSelectionFields
              controller={controller}
              selectedPitchOption={selectedPitchOption}
              selectedSizeOption={selectedSizeOption}
            />
          </FormGrid.Area>

          <FormGrid.Area name="work">
            <ThreadApprenticeWorkflow apprentice={apprentice} />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

function ThreadSelectionFields({
  controller,
  selectedPitchOption,
  selectedSizeOption,
}: {
  controller: ReturnType<typeof useThreadsPageController>;
  selectedPitchOption: ReturnType<
    typeof useThreadsPageController
  >["pitchOptions"][number] | undefined;
  selectedSizeOption: ReturnType<
    typeof useThreadsPageController
  >["sizeOptions"][number] | undefined;
}) {
  const { form, navigation } = controller;

  return (
    <div className="threads-selection-grid">
      <FormSelectMenuField
        label={threadSelectConfig.type.label}
        tooltip={threadSelectConfig.type.tooltip}
        valueLabel={
          controller.typeOptions.find((option) => option.value === controller.type)
            ?.label ?? "-"
        }
        options={controller.typeOptions}
        onSelect={controller.onTypeChange}
        disabled={controller.loadingOptions}
        staticWhenSingleOption
      />

      <FormSelectMenuField
        label={threadSelectConfig.size.label}
        tooltip={threadSelectConfig.size.tooltip}
        valueLabel={selectedSizeOption?.label ?? (form.fields.size.value || "-")}
        options={controller.sizeOptions}
        onSelect={controller.onSizeChange}
        disabled={controller.loadingOptions}
        staticWhenSingleOption
        ref={navigation.register("size")}
        onKeyDown={navigation.handleKeyDown("size")}
      />

      <FormSelectMenuField
        label={threadSelectConfig.pitch.label}
        tooltip={threadSelectConfig.pitch.tooltip}
        valueLabel={
          selectedPitchOption ? (
            <SelectMenuLabel
              label={selectedPitchOption.label}
              meta={selectedPitchOption.meta}
            />
          ) : (
            "-"
          )
        }
        options={controller.pitchOptions}
        onSelect={controller.onPitchChange}
        disabled={controller.loadingOptions}
        staticWhenSingleOption
        ref={navigation.register("pitch")}
        onKeyDown={navigation.handleKeyDown("pitch")}
      />
      {selectedPitchOption ? (
        <div className="threads-pitch-meta">
          {formatMillimeters(selectedPitchOption.pitchMm)} mm
        </div>
      ) : null}
    </div>
  );
}

function ThreadApprenticeWorkflow({
  apprentice,
}: {
  apprentice: ReturnType<typeof useThreadsPageController>["apprentice"];
}) {
  const { guide } = apprentice;

  if (!guide) {
    return (
      <div className="threads-apprentice-workflow">
        <p className="threads-apprentice-empty">
          Velg trådstørrelse og stigning for å starte lærlingflyten.
        </p>
      </div>
    );
  }

  return (
    <div className="threads-apprentice-workflow">
      <div className="threads-apprentice-intro">
        <h3>Lærlingmodus</h3>
        <p>
          Regn ut verdiene selv, sjekk svaret, og juster til beregningen
          stemmer.
        </p>
      </div>

      <ThreadApprenticeStep
        id="thread-apprentice-drill"
        title="1. Finn bor for gjenging"
        formula={guide.drillFormula}
        calculation={guide.drillCalculation}
        answer={apprentice.answers.drill}
        target={guide.drillDiameterMm}
        onChange={(value) => apprentice.updateAnswer("drill", value)}
        onCheck={() => apprentice.checkAnswer("drill")}
        onToggleWork={() => apprentice.toggleWork("drill")}
        onToggleAnswer={() => apprentice.toggleAnswer("drill")}
      />

      <ThreadApprenticeStep
        id="thread-apprentice-depth"
        title="2. Finn radial gjengedybde"
        formula={guide.depthFormula}
        calculation={guide.depthCalculation}
        answer={apprentice.answers.depth}
        target={guide.threadDepthMm}
        onChange={(value) => apprentice.updateAnswer("depth", value)}
        onCheck={() => apprentice.checkAnswer("depth")}
        onToggleWork={() => apprentice.toggleWork("depth")}
        onToggleAnswer={() => apprentice.toggleAnswer("depth")}
      />
    </div>
  );
}

function ThreadApprenticeStep({
  id,
  title,
  formula,
  calculation,
  answer,
  target,
  onChange,
  onCheck,
  onToggleWork,
  onToggleAnswer,
}: {
  id: string;
  title: string;
  formula: string;
  calculation: string;
  answer: ReturnType<
    typeof useThreadsPageController
  >["apprentice"]["answers"]["drill"];
  target: number;
  onChange: (value: string) => void;
  onCheck: () => void;
  onToggleWork: () => void;
  onToggleAnswer: () => void;
}) {
  const answerPlaceholder = answer.answerVisible
    ? `Fasit: ${formatMillimeters(target)} mm`
    : undefined;

  return (
    <section className={`threads-apprentice-step is-${answer.status}`}>
      <div className="threads-apprentice-step-header">
        <h4>{title}</h4>
      </div>

      <Field label="Ditt svar" htmlFor={id}>
        <NumberInput
          id={id}
          value={answer.value}
          onChange={onChange}
          unit="mm"
          appearance="form"
          source={answer.status === "correct" ? "machine" : "default"}
          placeholder={answerPlaceholder}
        />
      </Field>

      <div className="threads-apprentice-step-actions">
        <Button type="button" variant="secondary" size="small" onClick={onCheck}>
          Sjekk
        </Button>
        <Button type="button" variant="secondary" size="small" onClick={onToggleWork}>
          {answer.workVisible ? "Skjul fremgangsmåte" : "Vis fremgangsmåte"}
        </Button>
        <Button type="button" variant="link" size="small" onClick={onToggleAnswer}>
          {answer.answerVisible ? "Skjul fasit" : "Vis fasit"}
        </Button>
      </div>

      {answer.workVisible ? (
        <div className="threads-apprentice-work">
          <div className="threads-apprentice-work-header">
            <span>Fremgangsmåte</span>
            <code>{formula}</code>
          </div>
          <p className="threads-apprentice-calculation">{calculation}</p>
        </div>
      ) : null}

      {answer.feedback ? (
        <p className="threads-apprentice-feedback">
          {answer.feedback}
        </p>
      ) : null}
    </section>
  );
}

function formatMillimeters(value: number) {
  return value.toFixed(3);
}

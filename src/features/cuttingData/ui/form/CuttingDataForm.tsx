import { FormActions } from "@shared/ui/form/FormActions";
import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { Field } from "@shared/ui/form/Field";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { FormNumberFields } from "@shared/ui/form/fields/FormNumberFields";
import { Button } from "@shared/ui/primitives/Button/Button";
import { NumberInput } from "@shared/ui/primitives/input";

import { cuttingDataFieldConfig } from "../cuttingDataFieldConfig";
import type { CuttingDataKey } from "../../domain/cuttingDataForm";
import type { useCuttingPageController } from "../useCuttingPageController";
import "../CuttingDataPage.css";

type Props = {
  controller: ReturnType<typeof useCuttingPageController>;
};

export function CuttingDataForm({ controller }: Props) {
  const { form, navigation, save } = controller;

  const error = form.formError ? <FormError error={form.formError} /> : null;

  if (controller.apprentice.enabled) {
    return <CuttingDataApprenticeForm controller={controller} error={error} />;
  }

  return (
    <div ref={navigation.containerRef} className="cutting-data-form-root">
      <FormLayout
        error={error}
        actions={
          <FormActions
            onCalculate={controller.calculate}
            onReset={controller.resetForm}
          >
            <Button variant="secondary" size="medium" onClick={save}>
              Save result
            </Button>
          </FormActions>
        }
        actionsPlacement="bottom"
      >
        <FormGrid areas={[["fields"]]}>
          <FormGrid.Area name="fields">
            <FormNumberFields
              configs={cuttingDataFieldConfig}
              fields={form.fields}
              onChange={controller.onFieldChange}
              register={navigation.register}
              onKeyDown={navigation.handleKeyDown}
            />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

function CuttingDataApprenticeForm({
  controller,
  error,
}: {
  controller: ReturnType<typeof useCuttingPageController>;
  error: React.ReactNode;
}) {
  const { form, navigation, apprentice, save } = controller;
  const setupConfigs = cuttingDataFieldConfig.filter((config) => {
    if (!apprentice.selectedTarget) return false;
    if (!apprentice.requiredInputKeys.includes(config.key)) return false;

    return form.status !== "solved" || form.fields[config.key].source !== "machine";
  });

  return (
    <div ref={navigation.containerRef} className="cutting-data-form-root">
      <FormLayout
        error={error}
        actions={
          <div className="cutting-apprentice-actions">
            <Button
              variant="primary"
              size="large"
              onClick={controller.calculate}
              disabled={!apprentice.selectedTarget}
            >
              Lag oppgaver
            </Button>
            <Button
              variant="secondary"
              size="medium"
              onClick={apprentice.checkAll}
              disabled={apprentice.tasks.length === 0}
            >
              Sjekk svar
            </Button>
            <Button
              variant="secondary"
              size="medium"
              onClick={save}
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
        <FormGrid areas={[["setup"], ["work"]]}>
          <FormGrid.Area name="setup">
            <div className="cutting-apprentice-setup">
              <CuttingApprenticeTargetChooser apprentice={apprentice} />

              {apprentice.selectedTarget ? (
                <>
                  <p className="cutting-apprentice-required-summary">
                    {getRequiredSummary(apprentice.selectedTarget)}
                  </p>
                  <div className="cutting-apprentice-setup-grid">
                    {setupConfigs.map((config) => {
                      const requirement = getFieldRequirement(
                        apprentice.selectedTarget,
                        config.key,
                      );

                      return (
                        <div
                          key={config.key}
                          className={`cutting-apprentice-field-requirement is-${requirement}`}
                        >
                          <FormNumberField
                            label={formatRequiredLabel(config.label, requirement)}
                            tooltip={config.tooltip}
                            unit={config.unit}
                            field={form.fields[config.key]}
                            onChange={(value) =>
                              controller.onFieldChange(config.key, value)
                            }
                            ref={navigation.register(config.key)}
                            onKeyDown={navigation.handleKeyDown(config.key)}
                          />
                        </div>
                      );
                    })}
                  </div>
                </>
              ) : null}
            </div>
          </FormGrid.Area>

          <FormGrid.Area name="work">
            <CuttingApprenticeWorkflow apprentice={apprentice} />
          </FormGrid.Area>
        </FormGrid>
      </FormLayout>
    </div>
  );
}

function CuttingApprenticeTargetChooser({
  apprentice,
}: {
  apprentice: ReturnType<typeof useCuttingPageController>["apprentice"];
}) {
  return (
    <div className="cutting-apprentice-targets">
      <div className="cutting-apprentice-intro">
        <h3>Lærlingmodus</h3>
        <p>Velg først hva du vil regne ut. Deretter viser appen hvilke felt du trenger.</p>
      </div>

      <div className="cutting-apprentice-target-grid">
        {apprentice.targetOptions.map((option) => (
          <button
            key={option.key}
            type="button"
            className={`cutting-apprentice-target ${
              apprentice.selectedTarget === option.key ? "is-selected" : ""
            }`}
            onClick={() => apprentice.setSelectedTarget(option.key)}
          >
            <span>{option.label}</span>
            <small>{option.description}</small>
          </button>
        ))}
      </div>
    </div>
  );
}

function CuttingApprenticeWorkflow({
  apprentice,
}: {
  apprentice: ReturnType<typeof useCuttingPageController>["apprentice"];
}) {
  if (apprentice.tasks.length === 0) {
    return (
      <div className="cutting-apprentice-workflow">
        <div className="cutting-apprentice-intro">
          <p>
            Velg hva du vil regne ut, fyll inn feltene, og trykk Lag oppgaver.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="cutting-apprentice-workflow">
      <div className="cutting-apprentice-intro">
        <p>Regn ut hvert skjulte felt, sjekk svaret, og prøv igjen ved behov.</p>
      </div>

      {apprentice.tasks.map((task, index) => (
        <CuttingApprenticeStep
          key={task.key}
          id={`cutting-apprentice-${task.key}`}
          title={`${index + 1}. ${task.title}`}
          unit={task.unit}
          task={task}
          answer={apprentice.answers[task.key]}
          onChange={(value) => apprentice.updateAnswer(task.key, value)}
          onCheck={() => apprentice.checkAnswer(task.key)}
          onToggleWork={() => apprentice.toggleWork(task.key)}
          onToggleAnswer={() => apprentice.toggleAnswer(task.key)}
        />
      ))}
    </div>
  );
}

function CuttingApprenticeStep({
  id,
  title,
  unit,
  task,
  answer,
  onChange,
  onCheck,
  onToggleWork,
  onToggleAnswer,
}: {
  id: string;
  title: string;
  unit: string | undefined;
  task: ReturnType<typeof useCuttingPageController>["apprentice"]["tasks"][number];
  answer:
    | ReturnType<typeof useCuttingPageController>["apprentice"]["answers"][CuttingDataKey]
    | undefined;
  onChange: (value: string) => void;
  onCheck: () => void;
  onToggleWork: () => void;
  onToggleAnswer: () => void;
}) {
  const current = answer ?? {
    value: "",
    status: "idle" as const,
    feedback: "",
    attempts: 0,
    workVisible: false,
    answerVisible: false,
  };
  const answerPlaceholder = current.answerVisible
    ? `Fasit: ${formatValue(task.target)}${unit ? ` ${unit}` : ""}`
    : undefined;

  return (
    <section className={`cutting-apprentice-step is-${current.status}`}>
      <div className="cutting-apprentice-step-header">
        <h4>{title}</h4>
      </div>
      <p className="cutting-apprentice-description">{task.description}</p>

      <Field label="Ditt svar" htmlFor={id}>
        <NumberInput
          id={id}
          value={current.value}
          onChange={onChange}
          unit={unit}
          appearance="form"
          source={current.status === "correct" ? "machine" : "default"}
          placeholder={answerPlaceholder}
        />
      </Field>

      <div className="cutting-apprentice-step-actions">
        <Button type="button" variant="secondary" size="small" onClick={onCheck}>
          Sjekk
        </Button>
        <Button type="button" variant="secondary" size="small" onClick={onToggleWork}>
          {current.workVisible ? "Skjul fremgangsmåte" : "Vis fremgangsmåte"}
        </Button>
        <Button type="button" variant="link" size="small" onClick={onToggleAnswer}>
          {current.answerVisible ? "Skjul fasit" : "Vis fasit"}
        </Button>
      </div>

      {current.workVisible ? (
        <div className="cutting-apprentice-work">
          <div className="cutting-apprentice-work-header">
            <span>Fremgangsmåte</span>
            <code>{task.formula}</code>
          </div>
          <p className="cutting-apprentice-calculation">{task.calculation}</p>
        </div>
      ) : null}

      {current.feedback ? (
        <p className="cutting-apprentice-feedback">{current.feedback}</p>
      ) : null}
    </section>
  );
}

function formatValue(value: number) {
  return value.toFixed(3);
}

function getRequiredSummary(
  target: ReturnType<typeof useCuttingPageController>["apprentice"]["selectedTarget"],
) {
  if (target === "rpm") return "Obligatorisk: D og Vc.";
  if (target === "cutting_speed") return "Obligatorisk: D og n.";
  if (target === "feed_rate") {
    return "Obligatorisk: D, z og fz, pluss enten n eller Vc.";
  }
  if (target === "chip_load") {
    return "Obligatorisk: D, z og F, pluss enten n eller Vc.";
  }

  return "";
}

function getFieldRequirement(
  target: ReturnType<typeof useCuttingPageController>["apprentice"]["selectedTarget"],
  key: CuttingDataKey,
) {
  if (target === "rpm") {
    return key === "diameter" || key === "cutting_speed" ? "required" : "none";
  }

  if (target === "cutting_speed") {
    return key === "diameter" || key === "rpm" ? "required" : "none";
  }

  if (target === "feed_rate") {
    if (key === "diameter" || key === "teeth" || key === "chip_load") {
      return "required";
    }
    if (key === "rpm" || key === "cutting_speed") {
      return "alternative";
    }
  }

  if (target === "chip_load") {
    if (key === "diameter" || key === "teeth" || key === "feed_rate") {
      return "required";
    }
    if (key === "rpm" || key === "cutting_speed") {
      return "alternative";
    }
  }

  return "none";
}

function formatRequiredLabel(label: string, requirement: string) {
  if (requirement === "required") return `${label} (obligatorisk)`;
  if (requirement === "alternative") return `${label} (alternativ)`;

  return label;
}

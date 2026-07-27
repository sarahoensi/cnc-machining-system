import { FormError } from "@shared/ui/form/FormError";
import { FormGrid } from "@shared/ui/form/FormGrid";
import { FormModeField } from "@shared/ui/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/form/fields/FormSelectMenuField";
import { Row } from "@shared/ui/primitives/Row/Row";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/hooks";

import {
  toleranceClassFieldConfig,
  toleranceFieldConfig,
  toleranceModeConfig,
} from "../toleranceFieldConfig";
import type { ToleranceFormState, ToleranceKey } from "../../domain/toleranceForm";
import type { ToleranceObjectType } from "../../api/types";
import type { useTolerancePageController } from "../useTolerancePageController";
import "./TolerancesForm.css";

const modeOptions = [
  { value: "hole", label: "Hole" },
  { value: "shaft", label: "Shaft" },
] as const;

const toleranceNavigationKeys = [
  "nominal",
  "hole_letter",
  "hole_grade",
  "shaft_letter",
  "shaft_grade",
] as const;

type ToleranceNavigationKey = Extract<
  ToleranceKey,
  (typeof toleranceNavigationKeys)[number]
>;

type ToleranceClassNavigationKey = Exclude<ToleranceNavigationKey, "nominal">;

type ToleranceController = ReturnType<typeof useTolerancePageController>;

type Props = {
  controller: ToleranceController;
};

const toleranceFieldConfigByKey = Object.fromEntries(
  toleranceFieldConfig.map((fieldConfig) => [fieldConfig.key, fieldConfig]),
) as Record<ToleranceKey, (typeof toleranceFieldConfig)[number]>;

export function TolerancesForm({ controller }: Props) {
  const {
    form,
    mode,
    loadingOptions,

    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,

    holeLetterOptions,
    holeGradeOptions,
    shaftLetterOptions,
    shaftGradeOptions,
  } = controller;

  const navigation = useFormNavigation<ToleranceNavigationKey>({
    keys: toleranceNavigationKeys,
    autoFocusOnMount: true,
    activePath: "/tolerances",
    onSubmit: onCalculate,
  });

  async function onCalculate() {
    const next = await controller.calculate();
    if (!next.formError) return;

    navigation.focusFirstInOrderAfterRender(toleranceNavigationKeys, (key) =>
      shouldFocusMissingToleranceField(next, key),
    );
  }

  function onReset() {
    controller.resetForm();
    navigation.focusFirstAfterRender();
  }

  function renderNumberField(key: ToleranceKey) {
    const fieldConfig = toleranceFieldConfigByKey[key];
    const fieldState = form.fields[fieldConfig.key];

    return (
      <FormNumberField
        key={fieldConfig.key}
        label={fieldConfig.label}
        tooltip={fieldConfig.tooltip}
        unit={fieldConfig.unit}
        field={fieldState}
        autoFocus={fieldConfig.autoFocus}
        disabled={fieldState.locked}
        readonly={fieldConfig.readOnly}
        onChange={(value) => controller.onFieldChange(fieldConfig.key, value)}
        ref={fieldConfig.key === "nominal" ? navigation.register("nominal") : undefined}
        onKeyDown={
          fieldConfig.key === "nominal"
            ? navigation.handleKeyDown("nominal")
            : undefined
        }
      />
    );
  }

  const classFields = getActiveClassFields({
    mode,
    loadingOptions,
    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,
    holeLetterOptions,
    holeGradeOptions,
    shaftLetterOptions,
    shaftGradeOptions,
  });

  const error = form.formError ? <FormError error={form.formError} /> : null;

  return (
    <div className="tolerances-form-container" ref={navigation.containerRef}>
      <FormGrid
        areas={[["mode"], ["input"], ["deviation"], ["limits"], ["error"], ["actions"]]}
        className="tolerances-form-content"
      >
        <FormGrid.Area name="mode">
          <Row columns={1} className="tolerances-mode-row">
            <FormModeField
              label={toleranceModeConfig.label}
              tooltip={toleranceModeConfig.tooltip}
              value={mode}
              options={modeOptions}
              onChange={controller.onModeChange}
            />
          </Row>
        </FormGrid.Area>

        <FormGrid.Area name="input">
          <FormGrid
            areas={[["fields", "calculate"]]}
            collapsedAreas={[["fields"], ["calculate"]]}
            columns="minmax(0, 1fr) auto"
            columnGap="var(--space-3)"
            align="end"
            className="tolerances-input-row"
          >
            <FormGrid.Area name="fields">
              <div className="tolerances-input-row-fields">
                <div className="tolerances-input-field">
                  {renderNumberField("nominal")}
                </div>

                {classFields.map((field) => (
                  <div className="tolerances-input-field" key={field.key}>
                    <FormSelectMenuField
                      label={field.label}
                      tooltip={field.tooltip}
                      valueLabel={field.valueLabel}
                      options={field.options}
                      onSelect={(value) =>
                        field.kind === "letter"
                          ? controller.onToleranceLetterChange(field.feature, value)
                          : controller.onToleranceGradeChange(field.feature, value)
                      }
                      disabled={field.disabled}
                      ref={navigation.register(field.key)}
                      onKeyDown={navigation.handleKeyDown(field.key)}
                    />
                  </div>
                ))}
              </div>
            </FormGrid.Area>

            <FormGrid.Area name="calculate" className="tolerances-calculate-slot">
              <Button
                ref={navigation.registerSubmitAction}
                variant="primary"
                size="large"
                onClick={onCalculate}
                onKeyDown={navigation.handleSubmitActionKeyDown}
              >
                Calculate
              </Button>
            </FormGrid.Area>
          </FormGrid>
        </FormGrid.Area>

        <FormGrid.Area name="deviation">
          <Row columns={2} className="tolerances-deviation-row">
            {renderNumberField("upper_um")}
            {renderNumberField("lower_um")}
          </Row>
        </FormGrid.Area>

        <FormGrid.Area name="limits">
          <Row columns={2} className="tolerances-limit-row">
            {renderNumberField("max_mm")}
            {renderNumberField("min_mm")}
          </Row>
        </FormGrid.Area>

        <FormGrid.Area name="error">{error}</FormGrid.Area>

        <FormGrid.Area name="actions">
          <div className="tolerances-actions">
            <Button
              variant="secondary"
              size="medium"
              onClick={controller.save}
              disabled={form.status !== "solved"}
            >
              Save result
            </Button>

            <Button variant="danger" size="medium" onClick={onReset}>
              Clear form
            </Button>
          </div>
        </FormGrid.Area>
      </FormGrid>
    </div>
  );
}

function shouldFocusMissingToleranceField(
  form: ToleranceFormState,
  key: ToleranceNavigationKey,
) {
  if (key === "hole_letter" || key === "hole_grade") {
    return form.extras.mode === "hole" && !form.fields[key].value.trim();
  }

  if (key === "shaft_letter" || key === "shaft_grade") {
    return form.extras.mode === "shaft" && !form.fields[key].value.trim();
  }

  return !form.fields[key].value.trim();
}

function getActiveClassFields({
  mode,
  loadingOptions,
  holeLetter,
  holeGrade,
  shaftLetter,
  shaftGrade,
  holeLetterOptions,
  holeGradeOptions,
  shaftLetterOptions,
  shaftGradeOptions,
}: Pick<
  ToleranceController,
  | "mode"
  | "loadingOptions"
  | "holeLetter"
  | "holeGrade"
  | "shaftLetter"
  | "shaftGrade"
  | "holeLetterOptions"
  | "holeGradeOptions"
  | "shaftLetterOptions"
  | "shaftGradeOptions"
>) {
  const feature: ToleranceObjectType = mode;
  const isHole = mode === "hole";
  const letterOptions = isHole ? holeLetterOptions : shaftLetterOptions;
  const gradeOptions = isHole ? holeGradeOptions : shaftGradeOptions;

  return [
    {
      key: `${mode}_letter` as ToleranceClassNavigationKey,
      kind: "letter" as const,
      feature,
      label: "Class",
      tooltip: toleranceClassFieldConfig.classTooltip,
      valueLabel: (isHole ? holeLetter : shaftLetter) || "-",
      options: letterOptions,
      disabled: loadingOptions,
    },
    {
      key: `${mode}_grade` as ToleranceClassNavigationKey,
      kind: "grade" as const,
      feature,
      label: "Grade",
      tooltip: toleranceClassFieldConfig.gradeTooltip,
      valueLabel: (isHole ? holeGrade : shaftGrade) || "-",
      options: gradeOptions,
      disabled: loadingOptions || gradeOptions.length === 0,
    },
  ];
}

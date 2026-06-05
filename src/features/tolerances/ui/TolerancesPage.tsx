// features/tolerances/ui/TolerancesPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormError } from "@shared/ui/form/FormError";
import { FormModeField } from "@shared/ui/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/form/fields/FormSelectMenuField";
import { PageShell } from "@shared/ui/page/PageShell";
import { Row } from "@shared/ui/primitives/Row/Row";
import { Stack } from "@shared/ui/primitives/Stack/Stack";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/hooks";

import {
  toleranceClassFieldConfig,
  toleranceFieldConfig,
  toleranceModeConfig,
} from "./toleranceFieldConfig";
import { ToleranceHistoryPanel } from "./history/ToleranceHistoryPanel";
import type { ToleranceKey } from "../domain/toleranceForm";
import "./TolerancesPage.css";
import { useTolerancePageController } from "./useTolerancePageController";

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

export function TolerancesPage() {
  usePageTitle("Tolerances");

  const controller = useTolerancePageController();

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

    navigation.focusFirstInOrderAfterRender(toleranceNavigationKeys, (key) => {
      if (key === "hole_letter" || key === "hole_grade") {
        return next.extras.mode === "hole" && !next.fields[key].value.trim();
      }

      if (key === "shaft_letter" || key === "shaft_grade") {
        return next.extras.mode === "shaft" && !next.fields[key].value.trim();
      }

      return !next.fields[key].value.trim();
    });
  }

  function onReset() {
    controller.resetForm();
    navigation.focusFirstAfterRender();
  }

  function renderNumberField(key: ToleranceKey) {
    const fieldConfig = toleranceFieldConfig.find(
      (candidate) => candidate.key === key,
    )!;
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
        ref={
          fieldConfig.key === "nominal"
            ? navigation.register("nominal")
            : undefined
        }
        onKeyDown={
          fieldConfig.key === "nominal"
            ? navigation.handleKeyDown("nominal")
            : undefined
        }
      />
    );
  }

  const modeField = (
    <Row columns={1} className="tolerances-mode-row">
      <FormModeField
        label={toleranceModeConfig.label}
        tooltip={toleranceModeConfig.tooltip}
        value={mode}
        options={modeOptions}
        onChange={controller.onModeChange}
      />
    </Row>
  );

  const inputRow = (
    <div className="tolerances-input-row">
      <div className="tolerances-input-row-fields">
        <div className="tolerances-input-field">
          {renderNumberField("nominal")}
        </div>
        {mode === "hole" && (
          <>
            <div className="tolerances-input-field">
              <FormSelectMenuField
                label="Class"
                tooltip={toleranceClassFieldConfig.classTooltip}
                valueLabel={holeLetter || "-"}
                options={holeLetterOptions}
                onSelect={(value) =>
                  controller.onToleranceLetterChange("hole", value)
                }
                disabled={loadingOptions}
                ref={navigation.register("hole_letter")}
                onKeyDown={navigation.handleKeyDown("hole_letter")}
              />
            </div>

            <div className="tolerances-input-field">
              <FormSelectMenuField
                label="Grade"
                tooltip={toleranceClassFieldConfig.gradeTooltip}
                valueLabel={holeGrade || "-"}
                options={holeGradeOptions}
                onSelect={(value) =>
                  controller.onToleranceGradeChange("hole", value)
                }
                disabled={loadingOptions || holeGradeOptions.length === 0}
                ref={navigation.register("hole_grade")}
                onKeyDown={navigation.handleKeyDown("hole_grade")}
              />
            </div>
          </>
        )}

        {mode === "shaft" && (
          <>
            <div className="tolerances-input-field">
              <FormSelectMenuField
                label="Class"
                tooltip={toleranceClassFieldConfig.classTooltip}
                valueLabel={shaftLetter || "-"}
                options={shaftLetterOptions}
                onSelect={(value) =>
                  controller.onToleranceLetterChange("shaft", value)
                }
                disabled={loadingOptions}
                ref={navigation.register("shaft_letter")}
                onKeyDown={navigation.handleKeyDown("shaft_letter")}
              />
            </div>

            <div className="tolerances-input-field">
              <FormSelectMenuField
                label="Grade"
                tooltip={toleranceClassFieldConfig.gradeTooltip}
                valueLabel={shaftGrade || "-"}
                options={shaftGradeOptions}
                onSelect={(value) =>
                  controller.onToleranceGradeChange("shaft", value)
                }
                disabled={loadingOptions || shaftGradeOptions.length === 0}
                ref={navigation.register("shaft_grade")}
                onKeyDown={navigation.handleKeyDown("shaft_grade")}
              />
            </div>
          </>
        )}
      </div>

      <div className="tolerances-calculate-slot">
        <Button
          ref={navigation.registerSubmitAction}
          variant="primary"
          size="large"
          onClick={onCalculate}
          onKeyDown={navigation.handleSubmitActionKeyDown}
        >
          Calculate
        </Button>
      </div>
    </div>
  );

  const outputRows = (
    <>
      <Row columns={2} className="tolerances-deviation-row">
        {renderNumberField("upper_um")}
        {renderNumberField("lower_um")}
      </Row>
      <Row columns={2} className="tolerances-limit-row">
        {renderNumberField("max_mm")}
        {renderNumberField("min_mm")}
      </Row>
    </>
  );

  const error = form.formError ? <FormError error={form.formError} /> : null;

  const saveButton = (
    <Button
      variant="secondary"
      size="medium"
      onClick={controller.save}
      disabled={form.status !== "solved"}
    >
      Save result
    </Button>
  );

  const actions = (
    <div className="tolerances-actions">
      {saveButton}
      <Button variant="danger" size="medium" onClick={onReset}>
        Clear form
      </Button>
    </div>
  );

  const formContent = (
    <div className="tolerances-form-content">
      {modeField}
      {inputRow}
      {outputRows}
      {error}
      {actions}
    </div>
  );

  return (
    <PageShell className="tolerances-page-layout">
      <Stack className="tolerances-page-stack">
        <div
          className="tolerances-page-form tolerances-form-container"
          ref={navigation.containerRef}
        >
          {formContent}
        </div>
        <div className="tolerances-page-history">
        <ToleranceHistoryPanel
          history={controller.history}
          onLoad={controller.load}
          onDelete={controller.remove}
          onClear={controller.clear}
        />
        </div>
      </Stack>
    </PageShell>
  );
}


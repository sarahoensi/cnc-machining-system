// features/tolerances/ui/TolerancesPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/components/form/fields/FormSelectMenuField";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { SplitFormLayout } from "@shared/ui/layout/container/SplitFormLayout/SplitFormLayout";
import { FormSidebarLayout } from "@shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/ui";

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

  const input = (
    <FormSection>
      <FormModeField
        label={toleranceModeConfig.label}
        tooltip={toleranceModeConfig.tooltip}
        value={mode}
        options={modeOptions}
        onChange={controller.onModeChange}
      />

      {toleranceFieldConfig
        .filter((fieldConfig) => !fieldConfig.readOnly)
        .map((fieldConfig) => {
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
              onChange={(value) =>
                controller.onFieldChange(fieldConfig.key, value)
              }
              ref={navigation.register(
                fieldConfig.key as ToleranceNavigationKey,
              )}
              onKeyDown={navigation.handleKeyDown(
                fieldConfig.key as ToleranceNavigationKey,
              )}
            />
          );
        })}

      {mode === "hole" && (
        <>
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
        </>
      )}

      {mode === "shaft" && (
        <>
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
        </>
      )}
    </FormSection>
  );

  const output = (
    <FormSection>
      {toleranceFieldConfig
        .filter((fieldConfig) => fieldConfig.readOnly)
        .map((fieldConfig) => {
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
              onChange={(value) =>
                controller.onFieldChange(fieldConfig.key, value)
              }
            />
          );
        })}
    </FormSection>
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
    <FormActions
      onCalculate={onCalculate}
      onReset={onReset}
      calculateRef={navigation.registerSubmitAction}
      onCalculateKeyDown={navigation.handleSubmitActionKeyDown}
    >
      {saveButton}
    </FormActions>
  );

  const formContent = (
    <SplitFormLayout
      input={input}
      output={output}
      error={error}
      actions={actions}
      inputWidth="8rem"
      outputWidth="7.5rem"
      gap="var(--space-3)"
    />
  );

  return (
    <FormSidebarLayout
      className="tolerances-page-layout"
      formWidth="lg"
      fillHeight
      form={
        <div
          className="tolerances-form-container"
          ref={navigation.containerRef}
        >
          {formContent}
        </div>
      }
      sidebar={
        <ToleranceHistoryPanel
          history={controller.history}
          onLoad={controller.load}
          onDelete={controller.remove}
          onClear={controller.clear}
        />
      }
    />
  );
}
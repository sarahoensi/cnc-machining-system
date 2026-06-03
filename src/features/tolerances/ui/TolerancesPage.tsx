import { usePageTitle } from "@app/providers/TitleContextProvider";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/components/form/fields/FormSelectMenuField";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { FormSidebarLayout } from "@shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/ui";
import {
  buildFieldHistoryItems,
  type HistoryItem,
  type SavedResultEntry,
} from "@shared/savedResults";
import { SavedResultsPanel } from "@shared/ui/components/history/SavedResultsPanel";

import {
  toleranceClassFieldConfig,
  toleranceFieldConfig,
  toleranceModeConfig,
} from "./toleranceFieldConfig";
import { useTolerancePageController } from "./useTolerancePageController";
import type { ToleranceFormState, ToleranceKey } from "../domain/toleranceForm";
import type { ToleranceOption } from "../api/types";
import "./TolerancesPage.css";


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
  const { decimals } = useDisplaySettings();

  const controller = useTolerancePageController();
  const { form } = controller;
  const {
    mode,
    options,
    loadingOptions,
  } = form.extras;
  const holeLetter = form.fields.hole_letter.value;
  const holeGrade = form.fields.hole_grade.value;
  const shaftLetter = form.fields.shaft_letter.value;
  const shaftGrade = form.fields.shaft_grade.value;

  const navigation = useFormNavigation<ToleranceNavigationKey>({
    keys: toleranceNavigationKeys,
    autoFocusOnMount: true,
    activePath: "/tolerances",
    onSubmit: onCalculate,
  });

  async function onCalculate() {
    const next = await controller.calculate();
    if (!next) return;

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

  const fields = (
    <>
      <FormSection>
        <FormModeField
          label={toleranceModeConfig.label}
          tooltip={toleranceModeConfig.tooltip}
          value={mode}
          options={modeOptions}
          onChange={controller.onModeChange}
        />

        {toleranceFieldConfig
          .filter((f) => !f.readOnly)
          .map((f) => {
            const fieldState = form.fields[f.key];
            return (
              <FormNumberField
                key={f.key}
                label={f.label}
                tooltip={f.tooltip}
                unit={f.unit}
                field={fieldState}
                autoFocus={f.autoFocus}
                disabled={fieldState.locked}
                readonly={f.readOnly}
                onChange={(value) => controller.onFieldChange(f.key, value)}
                ref={navigation.register(f.key as ToleranceNavigationKey)}
                onKeyDown={navigation.handleKeyDown(
                  f.key as ToleranceNavigationKey,
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
              options={options.holes.map((option) => ({
                value: option.zone,
                label: option.zone,
              }))}
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
              options={gradesForZone(options.holes, holeLetter).map((value) => ({
                value,
                label: value,
              }))}
              onSelect={(value) =>
                controller.onToleranceGradeChange("hole", value)
              }
              disabled={
                loadingOptions || gradesForZone(options.holes, holeLetter).length === 0
              }
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
              options={options.shafts.map((option) => ({
                value: option.zone,
                label: option.zone,
              }))}
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
              options={gradesForZone(options.shafts, shaftLetter).map((value) => ({
                value,
                label: value,
              }))}
              onSelect={(value) =>
                controller.onToleranceGradeChange("shaft", value)
              }
              disabled={
                loadingOptions ||
                gradesForZone(options.shafts, shaftLetter).length === 0
              }
              ref={navigation.register("shaft_grade")}
              onKeyDown={navigation.handleKeyDown("shaft_grade")}
            />
          </>
        )}
      </FormSection>

      <FormSection variant="result">
        {toleranceFieldConfig
          .filter((f) => f.readOnly)
          .map((f) => {
            const fieldState = form.fields[f.key];
            return (
              <FormNumberField
                key={f.key}
                label={f.label}
                tooltip={f.tooltip}
                unit={f.unit}
                field={fieldState}
                autoFocus={f.autoFocus}
                disabled={fieldState.locked}
                readonly={f.readOnly}
                onChange={(value) => controller.onFieldChange(f.key, value)}
              />
            );
          })}
      </FormSection>
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
    <FormLayout fields={fields} error={error} actions={actions} />
  );

  return (
    <FormSidebarLayout
      className="tolerances-page-layout"
      form={<div ref={navigation.containerRef}>{formContent}</div>}
      sidebar={
        <SavedResultsPanel
          entries={controller.history}
          buildItems={(entry) => buildToleranceHistoryItems(entry, decimals)}
          onLoad={controller.load}
          onDelete={controller.remove}
          onClear={controller.clear}
        />
      }
    />
  );
}

function gradesForZone(options: ToleranceOption[], zone: string) {
  return (
    options.find((option) => option.zone === zone)?.grades.map(String) ?? []
  );
}

function buildToleranceHistoryItems(
  entry: SavedResultEntry<ToleranceFormState>,
  decimals: number,
): HistoryItem[] {
  const { form } = entry;
  const mode = form.extras.mode;
  const letterKey = mode === "hole" ? "hole_letter" : "shaft_letter";
  const gradeKey = mode === "hole" ? "hole_grade" : "shaft_grade";
  const toleranceClass = `${form.fields[letterKey].value}${form.fields[gradeKey].value}`;
  const fieldItems = buildFieldHistoryItems(
    form.fields,
    toleranceFieldConfig.filter(
      (config) =>
        config.key === "nominal" ||
        config.key === "upper_um" ||
        config.key === "lower_um" ||
        config.key === "min_mm" ||
        config.key === "max_mm",
    ),
    decimals,
  );

  return [
    fieldItems[0] ?? {
      label: "Nominal size",
      value: "-",
      unit: "mm",
    },
    {
      label: "Class",
      value: toleranceClass.trim() || "-",
    },
    ...fieldItems.slice(1),
  ];
}

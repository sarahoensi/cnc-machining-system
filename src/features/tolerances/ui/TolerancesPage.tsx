import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { Modal, ModalScrollArea } from "@shared/ui/components/overlay/Modal/Modal";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/ui";

import {
  toleranceInputFieldConfig,
  toleranceModeConfig,
} from "./toleranceFieldConfig";
import { ToleranceClassFields } from "./ToleranceClassFields";
import { ToleranceResultFields } from "./ToleranceResultFields";
import { ToleranceResultTable } from "./ToleranceResultTable";
import { useTolerancePageController } from "./useTolerancePageController";

import "./TolerancesPage.css";

const modeOptions = [
  { value: "hole", label: "Hole" },
  { value: "shaft", label: "Shaft" },
] as const;

export function TolerancesPage() {
  usePageTitle("Tolerances");

  const { decimals } = useDisplaySettings();
  const controller = useTolerancePageController();
  const { form } = controller;
  const {
    mode,
    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,
    options,
    loadingOptions,
  } = form.extras;

  const navigation = useFormNavigation({
    keys: toleranceInputFieldConfig.map((field) => field.key),
    autoFocusOnMount: true,
    activePath: "/tolerances",
    onSubmit: onCalculate,
  });

  async function onCalculate() {
    const errors = await controller.calculate();

    if (Object.keys(errors).length > 0) {
      navigation.focusFirstInvalidAfterRender((key) => Boolean(errors[key]));
    }
  }

  function onReset() {
    controller.resetForm();
    navigation.focusFirstAfterRender();
  }

  const inputFields = (
    <FormSection>
      <FormModeField
        label={toleranceModeConfig.label}
        tooltip={toleranceModeConfig.tooltip}
        value={mode}
        options={modeOptions}
        onChange={controller.onModeChange}
      />

      {toleranceInputFieldConfig.map((fieldConfig) => (
        <FormNumberField
          key={fieldConfig.key}
          label={fieldConfig.label}
          tooltip={fieldConfig.tooltip}
          unit={fieldConfig.unit}
          field={form.fields[fieldConfig.key]}
          autoFocus={fieldConfig.autoFocus}
          onChange={controller.onNominalChange}
          ref={navigation.register(fieldConfig.key)}
          onKeyDown={navigation.handleKeyDown(fieldConfig.key)}
        />
      ))}

      {mode === "hole" && (
        <ToleranceClassFields
          feature="hole"
          options={options.holes}
          letter={holeLetter}
          grade={holeGrade}
          disabled={loadingOptions}
          onLetterChange={(value) =>
            controller.onToleranceLetterChange("hole", value)
          }
          onGradeChange={(value) =>
            controller.onToleranceGradeChange("hole", value)
          }
        />
      )}

      {mode === "shaft" && (
        <ToleranceClassFields
          feature="shaft"
          options={options.shafts}
          letter={shaftLetter}
          grade={shaftGrade}
          disabled={loadingOptions}
          onLetterChange={(value) =>
            controller.onToleranceLetterChange("shaft", value)
          }
          onGradeChange={(value) =>
            controller.onToleranceGradeChange("shaft", value)
          }
        />
      )}
    </FormSection>
  );

  const error = form.formError ? <FormError error={form.formError} /> : null;
  const actions = <FormActions onCalculate={onCalculate} onReset={onReset} />;

  return (
    <>
      <div className="tolerances-page">
        <div className="tolerances-input-column">
          <div ref={navigation.containerRef}>
            <FormLayout fields={inputFields} error={error} actions={actions} />
          </div>
        </div>

        <div className="tolerances-output-column">
          <ToleranceResultFields form={form} />

          <div className="tolerances-result-actions">
            <Button
              variant="secondary"
              size="small"
              onClick={() => controller.setTableOpen(true)}
              disabled={!controller.result}
            >
              View in table
            </Button>
          </div>
        </div>
      </div>

      {controller.tableOpen && controller.result && (
        <Modal
          title="ISO 286 result table"
          onClose={() => controller.setTableOpen(false)}
          size="lg"
          height="auto"
        >
          <ModalScrollArea>
            <ToleranceResultTable
              result={controller.result}
              decimals={decimals}
            />
          </ModalScrollArea>
        </Modal>
      )}
    </>
  );
}

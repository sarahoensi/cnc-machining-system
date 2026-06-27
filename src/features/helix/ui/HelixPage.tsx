// features/helix/ui/HelixPage.tsx

import {
  handleUserEdit,
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { useCalculatorFormNavigation } from "@shared/hooks";
import { validateHelixForm } from "../domain/validateHelixForm";

import {
  createInitialHelixForm,
  HelixKey,
} from "../domain/helixForm";

import { parseHelix } from "../domain/parseHelix";
import { solveHelix } from "../api/solveHelix";
import { helixFieldConfig } from "./helixFieldConfig";
import { FormModeField } from "@shared/ui/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/form/fields/FormNumberField";

import {
  validHelixInputSets,
  mutuallyExclusiveHelixPairs,
} from "../domain/helixConstraints"

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { FormActions } from "@shared/ui/form/FormActions";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormError } from "@shared/ui/form/FormError";
import { FormLayout } from "@shared/ui/form/FormLayout";
import { Stack } from "@shared/ui/primitives/Stack/Stack";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";
import { helixTooltips } from "./helixTooltip";
import { HelixFigure } from "./helixFigure/HelixFigure";
/* ============================================================
   Component
============================================================ */

export function HelixPage() {

  usePageTitle("Helix");

  const [form, setForm] = useFeatureForm(
    "helix",
    createInitialHelixForm
  );

  const fieldOrder = helixFieldConfig.map((f) => f.key);
  const formFocus = useCalculatorFormNavigation({
    fieldOrder,
    activePath: "/helix",
    onSubmit: onCalculate,
    trackActiveField: true,
  });

  /* =========================
     Field change
  ========================= */

  function onFieldChange(
    key: HelixKey,
    value: string
  ) {
    setForm(prev =>
      handleUserEdit(prev,
        key,
        value,
        validHelixInputSets,
        mutuallyExclusiveHelixPairs)
    );
  }

    /* =========================
     Mode change
  ========================= */

  function onModeChange(newMode: typeof form.extras.mode) {
    setForm(prev =>
      handleModeChange(prev, {
        ...prev.extras,
        mode: newMode,
      })
    );
  }

  /* =========================
     Calculate
  ========================= */

  async function onCalculate() {

    const next = await handleCalculateAsync(
      form,
      parseHelix,
      (input) => solveHelix(input, form.extras.mode),
      validateHelixForm,
    );

    setForm(next);
    formFocus.focusAfterCalculate(next);
  }

  /* =========================
     Reset
  ========================= */

  function onReset() {
    setForm(createInitialHelixForm());
    formFocus.focusAfterReset();
  }

  /* =========================
     Render
  ========================= */

  const fields = (
  <>
    <Stack className="stack--form-section">
      <FormModeField
        label="Mode"
        tooltip={helixTooltips.mode}
        value={form.extras.mode}
        onChange={onModeChange}
        options={[
          { value: "Outer", label: "Outer" },
          { value: "Inner", label: "Inner" },
        ]}
      />
    </Stack>

    <Stack className="stack--form-section">
      {helixFieldConfig.map((config) => {
        const fieldState = form.fields[config.key];

        return (
          <FormNumberField
            key={config.key}
            label={config.label}
            tooltip={config.tooltip}
            unit={config.unit}
            field={fieldState}
            disabled={fieldState.locked || config.readOnly}
            autoFocus={config.autoFocus}
            onChange={(value) => onFieldChange(config.key, value)}
            ref={formFocus.register(config.key)}
            onKeyDown={formFocus.handleKeyDown(config.key)}
            onFocus={
              formFocus.onFieldFocus
                ? () => formFocus.onFieldFocus!(config.key)
                : undefined
            }
            onBlur={
              formFocus.onFieldBlur
                ? () => formFocus.onFieldBlur!()
                : undefined
            }
          />
        );
      })}
    </Stack>
  </>
);

const error = form.formError ? (
  <FormError error={form.formError} />
) : null;

const actions = (
  <FormActions
    onCalculate={onCalculate}
    onReset={onReset}
  />
);

const formContent = (
  <div ref={formFocus.containerRef}>
    <FormLayout
      error={error}
      actions={actions}
    >
      {fields}
    </FormLayout>
  </div>
);


  return (
    <FormWithSidePanel
      form={formContent}
      sidePanel={
        <HelixFigure
          mode={form.extras.mode}
          activeField={formFocus.activeField}
        />
      }
    />
  );
}


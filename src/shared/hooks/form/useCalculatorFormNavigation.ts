import { useState } from "react";

import type { FieldState } from "@shared/form/types/fields";
import { useFormNavigation } from "./useFormNavigation";

type CalculatorFocusForm<K extends string> = {
  fields: Record<K, FieldState>;
  formError?: unknown;
};

type CalculatorFocusIntent = "inline-error" | "form-error" | "none";

export function getCalculatorFocusIntent<K extends string>(
  form: CalculatorFocusForm<K>,
  fieldOrder: readonly K[],
): CalculatorFocusIntent {
  if (fieldOrder.some((key) => Boolean(form.fields[key].error))) {
    return "inline-error";
  }

  return form.formError ? "form-error" : "none";
}

export function useCalculatorFormNavigation<K extends string>({
  fieldOrder,
  activePath,
  onSubmit,
  trackActiveField = false,
}: {
  fieldOrder: readonly K[];
  activePath: string;
  onSubmit: () => void;
  trackActiveField?: boolean;
}) {
  const [activeField, setActiveField] = useState<K | null>(null);
  const navigation = useFormNavigation({
    keys: fieldOrder,
    autoFocusOnMount: true,
    activePath,
    onSubmit,
  });

  function focusAfterCalculate(form: CalculatorFocusForm<K>) {
    const intent = getCalculatorFocusIntent(form, fieldOrder);

    if (intent === "inline-error") {
      navigation.focusFirstInvalidAfterRender((key) =>
        Boolean(form.fields[key].error),
      );
      return;
    }

    if (intent === "form-error") {
      navigation.focusFirstInOrderAfterRender(fieldOrder, (key) => {
        const value = form.fields[key]?.value;
        return value == null || String(value).trim() === "";
      });
    }
  }

  function focusAfterReset() {
    navigation.focusFirstAfterRender();
  }

  return {
    ...navigation,
    activeField: trackActiveField ? activeField : null,
    onFieldFocus: trackActiveField ? setActiveField : undefined,
    onFieldBlur: trackActiveField ? () => setActiveField(null) : undefined,
    focusAfterCalculate,
    focusAfterReset,
  };
}

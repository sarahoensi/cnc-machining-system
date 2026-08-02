import { useFeatureForm } from "@app/providers/FormStateProvider";
import { handleCalculateAsync, handleUserEdit } from "@shared/form/engine/formEngine";
import { machineField } from "@shared/form/types/fields";
import { useFormNavigation } from "@shared/hooks";

import { solveCylinderWeight } from "../api/solveCylinderWeight";
import {
  createInitialCylinderWeightForm,
  type CylinderWeightKey,
} from "../domain/cylinderWeightForm";
import { parseCylinderWeight } from "../domain/parseCylinderWeight";
import { validateCylinderWeightForm } from "../domain/validateCylinderWeightForm";

const validInputSets = [
  ["outer_diameter_mm", "inner_diameter_mm", "length_mm"],
] as const;

const mutuallyExclusivePairs = [] as const;

const focusOrder: Exclude<CylinderWeightKey, "mass_kg">[] = [
  "outer_diameter_mm",
  "inner_diameter_mm",
  "length_mm",
];

export function useCylinderWeightCalculation() {
  const [form, setForm] = useFeatureForm(
    "cylinder_weight",
    createInitialCylinderWeightForm,
  );

  const navigation = useFormNavigation({
    keys: focusOrder,
    autoFocusOnMount: true,
    activePath: "/cylinder-weight",
    onSubmit: calculate,
  });

  function onFieldChange(key: CylinderWeightKey, value: string) {
    setForm((prev) =>
      handleUserEdit(prev, key, value, validInputSets, mutuallyExclusivePairs),
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseCylinderWeight,
      solveCylinderWeight,
      validateCylinderWeightForm,
    );
    if (next.status === "solved") {
      const massField = next.fields.mass_kg;
      const hasMachineMass =
        massField.machineValue != null || massField.value.trim() !== "";

      if (hasMachineMass) {
        setForm({
          ...next,
          fields: {
            ...next.fields,
            mass_kg: machineField(
              massField.machineValue != null
                ? String(massField.machineValue)
                : massField.value,
              {
                ...massField,
                source: "machine",
              },
            ),
          },
        });
        focusAfterCalculate(next);
        return next;
      }
    }

    setForm(next);
    focusAfterCalculate(next);
    return next;
  }

  function resetForm() {
    setForm(resetCylinderWeightFormKeepingMaterial);
    navigation.focusFirstAfterRender();
  }

  function focusAfterCalculate(
    next: ReturnType<typeof createInitialCylinderWeightForm>,
  ) {
    const hasInlineError = focusOrder.some((key) => Boolean(next.fields[key].error));

    if (hasInlineError) {
      navigation.focusFirstInvalidAfterRender((key) => Boolean(next.fields[key].error));
      return;
    }

    if (!next.formError) return;

    navigation.focusFirstInOrderAfterRender(focusOrder, (key) => {
      const value = next.fields[key]?.value;
      return value == null || String(value).trim() === "";
    });
  }

  return {
    form,
    setForm,
    navigation,
    onFieldChange,
    calculate,
    resetForm,
  };
}

export function resetCylinderWeightFormKeepingMaterial(
  previous: ReturnType<typeof createInitialCylinderWeightForm>,
) {
  const initial = createInitialCylinderWeightForm();
  return {
    ...initial,
    extras: {
      ...initial.extras,
      materialId: previous.extras.materialId,
      materialName: previous.extras.materialName,
      densityKgM3: previous.extras.densityKgM3,
    },
  };
}

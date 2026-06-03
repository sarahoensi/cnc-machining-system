import { FormSelectMenuField } from "@shared/ui/components/form/fields/FormSelectMenuField";

import type { ToleranceObjectType, ToleranceOption } from "../api/types";
import { toleranceClassFieldConfig } from "./toleranceFieldConfig";

export function ToleranceClassFields({
  feature,
  options,
  letter,
  grade,
  disabled,
  onLetterChange,
  onGradeChange,
}: {
  feature: ToleranceObjectType;
  options: ToleranceOption[];
  letter: string;
  grade: string;
  disabled: boolean;
  onLetterChange: (value: string) => void;
  onGradeChange: (value: string) => void;
}) {
  const grades = gradesForZone(options, letter);
  const title = feature === "hole" ? "Hole" : "Shaft";

  return (
    <>
      <FormSelectMenuField
        label={`${title} class`}
        tooltip={toleranceClassFieldConfig.classTooltip}
        valueLabel={letter || "-"}
        options={options.map((option) => ({
          value: option.zone,
          label: option.zone,
        }))}
        onSelect={onLetterChange}
        disabled={disabled}
      />

      <FormSelectMenuField
        label={`${title} grade`}
        tooltip={toleranceClassFieldConfig.gradeTooltip}
        valueLabel={grade || "-"}
        options={grades.map((value) => ({
          value,
          label: value,
        }))}
        onSelect={onGradeChange}
        disabled={disabled || grades.length === 0}
      />
    </>
  );
}

function gradesForZone(options: ToleranceOption[], zone: string) {
  return (
    options.find((option) => option.zone === zone)?.grades.map(String) ?? []
  );
}

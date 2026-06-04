import { ToleranceFormState } from "@features/tolerances/domain/toleranceForm";
import { patchSelectionFields } from "@features/tolerances/domain/toleranceSelection";
import { useEffect } from "react";



export function useValidToleranceGrades(
  setForm: React.Dispatch<React.SetStateAction<ToleranceFormState>>,
  holeGrade: string,
  holeGrades: string[],
  shaftGrade: string,
  shaftGrades: string[],
) {
  useEffect(() => {
    if (holeGrades.length === 0 || holeGrades.includes(holeGrade)) return;

    setForm((prev) =>
      patchSelectionFields(prev, {
        hole_grade: holeGrades[0],
      }),
    );
  }, [holeGrade, holeGrades, setForm]);

  useEffect(() => {
    if (shaftGrades.length === 0 || shaftGrades.includes(shaftGrade)) return;

    setForm((prev) =>
      patchSelectionFields(prev, {
        shaft_grade: shaftGrades[0],
      }),
    );
  }, [shaftGrade, shaftGrades, setForm]);
}
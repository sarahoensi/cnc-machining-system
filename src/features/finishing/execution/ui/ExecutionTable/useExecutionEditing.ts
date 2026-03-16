// features/execution/ui/useExecutionEditing.ts

import { useState } from "react";

import { parseDecimalInput } from "@shared/parsing/decimalParser";
import { getTauriCommandError } from "@shared/api/tauriError";

type RegisterMeasurement = (
  step: number,
  measurement: number
) => Promise<void>;

export function useExecutionEditing(
  onRegisterMeasurement: RegisterMeasurement
) {

  const [editingStep, setEditingStep] =
    useState<number | null>(null);

  const [drafts, setDrafts] =
    useState<Record<number, string>>({});

  const [errors, setErrors] =
    useState<Record<number, string>>({});


  function updateDraft(step: number, value: string) {

    setDrafts(d => ({
      ...d,
      [step]: value,
    }));

    setErrors(e => {
      const next = { ...e };
      delete next[step];
      return next;
    });
  }


  function startEdit(step: number, value: string) {

    setDrafts(d => ({
      ...d,
      [step]: value ?? "",
    }));

    setEditingStep(step);
  }


  function cancelEdit() {
    setEditingStep(null);
  }


  async function confirmEdit(step: number) {

    const value = drafts[step];

    if (!value) return;

    const { normalized, number } =
      parseDecimalInput(value);

    if (number === null) {

      setErrors(e => ({
        ...e,
        [step]: "Invalid number",
      }));

      return;
    }

    try {

      await onRegisterMeasurement(
        step,
        number
      );

      setDrafts(d => ({
        ...d,
        [step]: normalized,
      }));

      setEditingStep(null);

    } catch (error) {

      const te = getTauriCommandError(error);
      const firstError =
        te?.fieldErrors?.[0];

      if (!firstError) return;

      setErrors(e => ({
        ...e,
        [step]: firstError.message,
      }));
    }
  }


  return {

    editingStep,
    drafts,
    errors,

    updateDraft,
    startEdit,
    cancelEdit,
    confirmEdit,

  };
}
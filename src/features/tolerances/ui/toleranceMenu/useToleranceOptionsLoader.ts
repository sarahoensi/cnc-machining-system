// features/tolerances/ui/toleranceMenu/useToleranceOptionsLoader.ts

import { useEffect } from "react";

import { getTauriCommandError } from "@shared/api/tauriError";

import { listIso286ToleranceOptionsApi } from "../../api/client";
import type { ToleranceFormState } from "../../domain/toleranceForm";
import { reconcileSelectionFields } from "../../domain/toleranceOptions";

type SetToleranceForm = React.Dispatch<
  React.SetStateAction<ToleranceFormState>
>;

export function useToleranceOptionsLoader(setForm: SetToleranceForm) {
  useEffect(() => {
    let cancelled = false;

    async function loadOptions() {
      setForm((prev) => ({
        ...prev,
        extras: {
          ...prev.extras,
          loadingOptions: true,
        },
        formError: undefined,
      }));

      try {
        const response = await listIso286ToleranceOptionsApi();
        if (cancelled) return;

        setForm((prev) => ({
          ...prev,
          fields: reconcileSelectionFields(prev.fields, response),
          extras: {
            ...prev.extras,
            options: response,
            loadingOptions: false,
          },
        }));
      } catch (error) {
        if (cancelled) return;

        setForm((prev) => ({
          ...prev,
          extras: {
            ...prev.extras,
            loadingOptions: false,
          },
          formError: getToleranceErrorMessage(error),
        }));
      }
    }

    void loadOptions();

    return () => {
      cancelled = true;
    };
  }, [setForm]);
}

function getToleranceErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);

  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;

  return "ISO 286 calculation failed";
}
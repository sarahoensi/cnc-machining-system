import { useEffect } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  clearMachineFields,
  handleCalculateAsync,
} from "@shared/form/engine/formEngine";
import { userField } from "@shared/form/types/fields";
import { useFormNavigation } from "@shared/hooks";
import { useSavedResults } from "@shared/savedResults";

import { listThreadOptionsApi } from "../api/client";
import { solveThread } from "../api/solveThread";
import type { ThreadType } from "../api/types";
import {
  createInitialThreadForm,
  type ThreadFormState,
  type ThreadKey,
} from "../domain/threadForm";
import {
  getThreadSize,
  getThreadSizes,
  getDefaultThreadPitch,
  reconcileThreadSelection,
  threadTypeOptions,
} from "../domain/threadOptions";
import { parseThread } from "../domain/parseThread";
import { validateThreadForm } from "../domain/validateThreadForm";

const navigationKeys = ["size", "pitch"] as const;

export function useThreadsPageController() {
  const [form, setForm] = useFeatureForm("threads", createInitialThreadForm);

  const savedResults = useSavedResults<ThreadFormState>({
    storageKey: "threads-history",
  });

  const navigation = useFormNavigation<ThreadKey>({
    keys: ["size", "pitch", "drill_diameter", "thread_depth"],
    autoFocusOnMount: true,
    activePath: "/threads",
    onSubmit: calculate,
  });

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
        const options = await listThreadOptionsApi();
        if (cancelled) return;

        setForm((prev) => {
          const withOptions = {
            ...prev,
            extras: {
              ...prev.extras,
              options,
              loadingOptions: false,
            },
          };

          return reconcileThreadSelection(withOptions, options);
        });
      } catch (error) {
        if (cancelled) return;

        setForm((prev) => ({
          ...prev,
          extras: {
            ...prev.extras,
            loadingOptions: false,
          },
          formError: getThreadErrorMessage(error),
        }));
      }
    }

    void loadOptions();

    return () => {
      cancelled = true;
    };
  }, [setForm]);

  const typeOptions = threadTypeOptions.map(toSelectOption);
  const sizeOptions = getThreadSizes(form.extras.options, form.extras.type).map(toSelectOption);
  const selectedSize = getThreadSize(
    form.extras.options,
    form.extras.type,
    form.fields.size.value,
  );
  const pitchOptions =
    selectedSize?.pitches.map((option) => ({
      value: option.value,
      label: buildPitchLabel(option.label, form.extras.type),
      meta: buildPitchMeta(option, form.extras.type),
      pitchMm: option.pitchMm,
    })) ?? [];

  function onTypeChange(value: ThreadType) {
    setForm((prev) => {
      const initialSize = getThreadSizes(prev.extras.options, value)[0];
      const initialPitch = getDefaultThreadPitch(initialSize);

      return {
        ...prev,
        status: "editing",
        extras: {
          ...prev.extras,
          type: value,
        },
        fields: {
          ...clearMachineFields(prev.fields),
          size: userField(initialSize?.value ?? ""),
          pitch: userField(initialPitch?.value ?? ""),
        },
        formError: undefined,
      };
    });
  }

  function onSizeChange(value: string) {
    setForm((prev) => {
      const size = getThreadSize(prev.extras.options, prev.extras.type, value);
      const defaultPitch = getDefaultThreadPitch(size);

      return {
        ...prev,
        status: "editing",
        fields: {
          ...clearMachineFields(prev.fields),
          size: userField(value),
          pitch: userField(defaultPitch?.value ?? ""),
        },
        formError: undefined,
      };
    });
  }

  function onPitchChange(value: string) {
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: {
        ...clearMachineFields(prev.fields),
        pitch: userField(value),
      },
      formError: undefined,
    }));
  }

  function onFieldChange(_key: ThreadKey, _value: string) {
    return;
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseThread,
      solveThread,
      validateThreadForm,
    );

    setForm(next);

    if (!next.formError) return;

    navigation.focusFirstInOrderAfterRender(
      navigationKeys,
      (key) => !next.fields[key].value.trim(),
    );
  }

  function resetForm() {
    setForm((prev) => {
      const initial = createInitialThreadForm();
      const withOptions = {
        ...initial,
        extras: {
          ...initial.extras,
          options: prev.extras.options,
          loadingOptions: prev.extras.loadingOptions,
        },
      };

      return reconcileThreadSelection(withOptions, prev.extras.options);
    });
    navigation.focusFirstAfterRender();
  }

  function save() {
    savedResults.save(form);
  }

  function load(entry: (typeof savedResults.history)[number]) {
    setForm(savedResults.load(entry));
  }

  return {
    form,
    navigation,
    type: form.extras.type,
    loadingOptions: form.extras.loadingOptions,
    typeOptions,
    sizeOptions,
    pitchOptions,
    onTypeChange,
    onSizeChange,
    onPitchChange,
    onFieldChange,
    calculate,
    resetForm,
    history: savedResults.history,
    save,
    load,
    remove: savedResults.remove,
    clear: savedResults.clear,
  };
}

function toSelectOption<T extends string>(option: { value: T; label: string }) {
  return {
    value: option.value,
    label: option.label,
  };
}

function buildPitchLabel(label: string, type: ThreadType) {
  if (type === "metric") {
    return label.replace(/\s*mm$/i, "");
  }

  return label;
}

function buildPitchMeta(
  option: {
    pitchMm: number;
    series: string;
    isDefaultPitch: boolean;
    sourceType?: string;
  },
  type: ThreadType,
) {
  const pitchMm = "";

  if (type === "unified") {
    const series = formatSeries(option.series);
    const source = option.sourceType?.toUpperCase();

    return [series && source ? `${series} (${source})` : series, pitchMm]
      .filter(Boolean)
      .join(" · ");
  }

  if (type === "metric") {
    return formatSeries(option.series) ?? "";
  }

  return pitchMm;
}

function formatSeries(value: string) {
  if (!value) return undefined;

  const normalized = value.toLowerCase();
  if (normalized === "coarse") return "Coarse";
  if (normalized === "fine") return "Fine";

  return value.toUpperCase();
}

function getThreadErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);

  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;

  return "Thread calculation failed";
}

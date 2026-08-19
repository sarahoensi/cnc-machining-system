import { useEffect, useMemo, useState } from "react";

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { parseDecimalInput } from "@shared/parsing/decimalParser";

import type { CuttingDataKey } from "../../domain/cuttingDataForm";
import type { createInitialCuttingDataForm } from "../../domain/cuttingDataForm";

type ApprenticeAnswerStatus = "idle" | "correct" | "close" | "incorrect" | "revealed";
export type CuttingApprenticeTarget =
  | "rpm"
  | "cutting_speed"
  | "feed_rate"
  | "chip_load";

type CuttingApprenticeAnswer = {
  value: string;
  status: ApprenticeAnswerStatus;
  feedback: string;
  attempts: number;
  workVisible: boolean;
  answerVisible: boolean;
};

export type CuttingApprenticeTask = {
  key: CuttingDataKey;
  title: string;
  description: string;
  unit: string | undefined;
  target: number;
  formula: string;
  calculation: string;
};

const emptyAnswer: CuttingApprenticeAnswer = {
  value: "",
  status: "idle",
  feedback: "",
  attempts: 0,
  workVisible: false,
  answerVisible: false,
};

const labels: Record<CuttingDataKey, string> = {
  diameter: "Tool diameter D",
  teeth: "Toothcount z",
  rpm: "Rotations n",
  cutting_speed: "Cutting speed Vc",
  feed_rate: "Feed rate F",
  chip_load: "Chip load fz",
};

const units: Partial<Record<CuttingDataKey, string>> = {
  diameter: "mm",
  rpm: "rpm",
  cutting_speed: "m/min",
  feed_rate: "mm/min",
  chip_load: "mm/tooth",
};

export const cuttingApprenticeTargetOptions: readonly {
  key: CuttingApprenticeTarget;
  label: string;
  description: string;
}[] = [
  {
    key: "rpm",
    label: "Rotations n",
    description: "Find spindle speed from cutting speed and tool diameter.",
  },
  {
    key: "cutting_speed",
    label: "Cutting speed Vc",
    description: "Find surface speed from tool diameter and spindle speed.",
  },
  {
    key: "feed_rate",
    label: "Feed rate F",
    description: "Find table feed from chip load, spindle speed and toothcount.",
  },
  {
    key: "chip_load",
    label: "Chip load fz",
    description: "Find chip load per tooth from feed, spindle speed and toothcount.",
  },
];

export function useCuttingApprenticeController({
  form,
  onTargetChange,
}: {
  form: ReturnType<typeof createInitialCuttingDataForm>;
  onTargetChange?: (target: CuttingApprenticeTarget) => void;
}) {
  const { apprenticeMode } = useDisplaySettings();
  const [selectedTarget, setSelectedTarget] =
    useState<CuttingApprenticeTarget | null>(null);
  const tasks = useMemo(
    () => buildTasks(form, selectedTarget),
    [form, selectedTarget],
  );
  const taskSignature = tasks
    .map((task) => `${task.key}:${task.target}`)
    .join("|");
  const [answers, setAnswers] = useState<
    Partial<Record<CuttingDataKey, CuttingApprenticeAnswer>>
  >({});

  useEffect(() => {
    setAnswers(
      Object.fromEntries(tasks.map((task) => [task.key, emptyAnswer])) as Partial<
        Record<CuttingDataKey, CuttingApprenticeAnswer>
      >,
    );
  }, [taskSignature, tasks]);

  function updateAnswer(key: CuttingDataKey, value: string) {
    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...(prev[key] ?? emptyAnswer),
        value,
        status: "idle",
        feedback: "",
      },
    }));
  }

  function checkAnswer(key: CuttingDataKey) {
    const task = tasks.find((item) => item.key === key);
    if (!task) return;

    const current = answers[key] ?? emptyAnswer;
    const parsed = parseDecimalInput(current.value).number;
    const next = evaluateAnswer(parsed, task.target);

    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...(prev[key] ?? emptyAnswer),
        ...next,
        attempts: (prev[key]?.attempts ?? 0) + 1,
      },
    }));
  }

  function checkAll() {
    for (const task of tasks) {
      checkAnswer(task.key);
    }
  }

  function toggleWork(key: CuttingDataKey) {
    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...(prev[key] ?? emptyAnswer),
        workVisible: !(prev[key]?.workVisible ?? false),
      },
    }));
  }

  function toggleAnswer(key: CuttingDataKey) {
    setAnswers((prev) => {
      const current = prev[key] ?? emptyAnswer;

      return {
        ...prev,
        [key]: {
          ...current,
          status: current.answerVisible ? current.status : "revealed",
          feedback: current.answerVisible
            ? current.feedback
            : "Fasit vises som hint. Skriv fortsatt inn svaret selv.",
          answerVisible: !current.answerVisible,
        },
      };
    });
  }

  function selectTarget(target: CuttingApprenticeTarget) {
    setSelectedTarget(target);
    onTargetChange?.(target);
  }

  return {
    enabled: apprenticeMode,
    selectedTarget,
    setSelectedTarget: selectTarget,
    targetOptions: cuttingApprenticeTargetOptions,
    requiredInputKeys: selectedTarget
      ? getRequiredInputKeys(selectedTarget)
      : ([] as CuttingDataKey[]),
    tasks,
    answers,
    updateAnswer,
    checkAnswer,
    checkAll,
    toggleWork,
    toggleAnswer,
  };
}

function buildTasks(
  form: ReturnType<typeof createInitialCuttingDataForm>,
  selectedTarget: CuttingApprenticeTarget | null,
): CuttingApprenticeTask[] {
  const values = getNumericValues(form);

  return (Object.keys(form.fields) as CuttingDataKey[])
    .filter((key) => form.fields[key].source === "machine")
    .filter((key) => !selectedTarget || key === selectedTarget)
    .map((key) => buildTask(key, values, form.fields[key].machineValue))
    .filter((task): task is CuttingApprenticeTask => Boolean(task));
}

export function getRequiredInputKeys(
  target: CuttingApprenticeTarget,
): CuttingDataKey[] {
  if (target === "rpm") return ["diameter", "cutting_speed"];
  if (target === "cutting_speed") return ["diameter", "rpm"];
  if (target === "feed_rate") {
    return ["diameter", "teeth", "chip_load", "rpm", "cutting_speed"];
  }

  return ["diameter", "teeth", "feed_rate", "rpm", "cutting_speed"];
}

function buildTask(
  key: CuttingDataKey,
  values: Partial<Record<CuttingDataKey, number>>,
  target: number | undefined,
): CuttingApprenticeTask | null {
  if (target === undefined) return null;

  if (key === "rpm" && values.cutting_speed && values.diameter) {
    return {
      key,
      title: labels[key],
      description:
        "Rotations n is the spindle speed. It tells the machine how many revolutions the tool makes per minute.",
      unit: units[key],
      target,
      formula: "n = (1000 x Vc) / (pi x D)",
      calculation: `(1000 x ${fmt(values.cutting_speed)}) / (pi x ${fmt(
        values.diameter,
      )}) =`,
    };
  }

  if (key === "cutting_speed" && values.rpm && values.diameter) {
    return {
      key,
      title: labels[key],
      description:
        "Cutting speed Vc is the surface speed at the tool edge. It connects tool diameter with spindle speed.",
      unit: units[key],
      target,
      formula: "Vc = pi x D x n / 1000",
      calculation: `pi x ${fmt(values.diameter)} x ${fmt(values.rpm)} / 1000 =`,
    };
  }

  if (key === "feed_rate" && values.chip_load && values.rpm && values.teeth) {
    return {
      key,
      title: labels[key],
      description:
        "Feed rate F is how fast the machine moves through the cut per minute.",
      unit: units[key],
      target,
      formula: "F = fz x n x z",
      calculation: `${fmt(values.chip_load)} x ${fmt(values.rpm)} x ${fmt(
        values.teeth,
      )} =`,
    };
  }

  if (key === "chip_load" && values.feed_rate && values.rpm && values.teeth) {
    return {
      key,
      title: labels[key],
      description:
        "Chip load fz is how much material each tooth removes on each revolution.",
      unit: units[key],
      target,
      formula: "fz = F / (n x z)",
      calculation: `${fmt(values.feed_rate)} / (${fmt(values.rpm)} x ${fmt(
        values.teeth,
      )}) =`,
    };
  }

  return null;
}

function getNumericValues(
  form: ReturnType<typeof createInitialCuttingDataForm>,
): Partial<Record<CuttingDataKey, number>> {
  const values: Partial<Record<CuttingDataKey, number>> = {};

  for (const key of Object.keys(form.fields) as CuttingDataKey[]) {
    const field = form.fields[key];
    const parsed =
      field.machineValue ?? parseDecimalInput(field.value).number ?? undefined;

    if (parsed !== undefined) {
      values[key] = parsed;
    }
  }

  return values;
}

function evaluateAnswer(
  value: number | null,
  target: number,
): Pick<CuttingApprenticeAnswer, "status" | "feedback"> {
  if (value === null) {
    return {
      status: "incorrect",
      feedback: "Skriv inn et tall før du sjekker.",
    };
  }

  const delta = Math.abs(value - target);
  const tolerance = Math.max(0.005, Math.abs(target) * 0.001);
  const closeTolerance = Math.max(0.05, Math.abs(target) * 0.01);

  if (delta <= tolerance) {
    return {
      status: "correct",
      feedback: "Riktig. Svaret stemmer med beregnet verdi.",
    };
  }

  if (delta <= closeTolerance) {
    return {
      status: "close",
      feedback: "Nesten. Sjekk avrunding og antall desimaler.",
    };
  }

  return {
    status: "incorrect",
    feedback:
      value > target
        ? "For høyt. Prøv igjen, eller åpne fremgangsmåten hvis du står fast."
        : "For lavt. Prøv igjen, eller åpne fremgangsmåten hvis du står fast.",
  };
}

function fmt(value: number, decimals = 3) {
  return value.toFixed(decimals);
}

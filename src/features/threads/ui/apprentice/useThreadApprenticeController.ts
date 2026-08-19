import { useEffect, useMemo, useState } from "react";

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { parseDecimalInput } from "@shared/parsing/decimalParser";

import type { ThreadPitchOption, ThreadSizeOption, ThreadType } from "../../api/types";
import type { ThreadFormState } from "../../domain/threadForm";

type ApprenticeAnswerKey = "drill" | "depth";
type ApprenticeAnswerStatus = "idle" | "correct" | "close" | "incorrect" | "revealed";

type ThreadApprenticeGuide = {
  sizeLabel: string;
  pitchLabel: string;
  drillDiameterMm: number;
  threadDepthMm: number;
  drillFormula: string;
  drillCalculation: string;
  depthFormula: string;
  depthCalculation: string;
};

type ApprenticeAnswer = {
  value: string;
  status: ApprenticeAnswerStatus;
  feedback: string;
  attempts: number;
  workVisible: boolean;
  answerVisible: boolean;
};

const emptyAnswer: ApprenticeAnswer = {
  value: "",
  status: "idle",
  feedback: "",
  attempts: 0,
  workVisible: false,
  answerVisible: false,
};

export function useThreadApprenticeController({
  form,
  selectedSize,
  selectedPitch,
}: {
  form: ThreadFormState;
  selectedSize: ThreadSizeOption | undefined;
  selectedPitch: ThreadPitchOption | undefined;
}) {
  const { apprenticeMode } = useDisplaySettings();
  const [answers, setAnswers] = useState<Record<ApprenticeAnswerKey, ApprenticeAnswer>>({
    drill: emptyAnswer,
    depth: emptyAnswer,
  });

  const guide = useMemo(
    () =>
      buildThreadApprenticeGuide({
        type: form.extras.type,
        sizeLabel: selectedSize?.label,
        majorDiameterMm: selectedSize?.majorDiameterMm,
        pitchLabel: selectedPitch?.label,
        pitchMm: selectedPitch?.pitchMm,
        drillDiameterMm: form.fields.drill_diameter.machineValue,
        threadDepthMm: form.fields.thread_depth.machineValue,
      }),
    [
      form.extras.type,
      form.fields.drill_diameter.machineValue,
      form.fields.thread_depth.machineValue,
      selectedPitch?.label,
      selectedPitch?.pitchMm,
      selectedSize?.label,
      selectedSize?.majorDiameterMm,
    ],
  );
  const guideSignature = guide
    ? [
        form.extras.type,
        guide.sizeLabel,
        guide.pitchLabel,
        guide.drillDiameterMm,
        guide.threadDepthMm,
      ].join("|")
    : "";

  useEffect(() => {
    setAnswers({
      drill: emptyAnswer,
      depth: emptyAnswer,
    });
  }, [guideSignature]);

  function updateAnswer(key: ApprenticeAnswerKey, value: string) {
    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        value,
        status: "idle",
        feedback: "",
      },
    }));
  }

  function checkAnswer(key: ApprenticeAnswerKey) {
    if (!guide) return;

    const target = key === "drill" ? guide.drillDiameterMm : guide.threadDepthMm;
    const parsed = parseDecimalInput(answers[key].value).number;
    const next = evaluateAnswer(parsed, target);

    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        ...next,
        attempts: prev[key].attempts + 1,
      },
    }));
  }

  function toggleWork(key: ApprenticeAnswerKey) {
    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        workVisible: !prev[key].workVisible,
      },
    }));
  }

  function toggleAnswer(key: ApprenticeAnswerKey) {
    setAnswers((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        status: prev[key].answerVisible ? prev[key].status : "revealed",
        feedback: prev[key].answerVisible
          ? prev[key].feedback
          : "Fasit vises som hint. Skriv fortsatt inn svaret selv.",
        answerVisible: !prev[key].answerVisible,
      },
    }));
  }

  return {
    enabled: apprenticeMode,
    guide,
    answers,
    updateAnswer,
    checkAnswer,
    toggleWork,
    toggleAnswer,
  };
}

function evaluateAnswer(
  value: number | null,
  target: number,
): Pick<ApprenticeAnswer, "status" | "feedback"> {
  if (value === null) {
    return {
      status: "incorrect",
      feedback: "Skriv inn et tall før du sjekker.",
    };
  }

  const delta = Math.abs(value - target);

  if (delta <= 0.005) {
    return {
      status: "correct",
      feedback: "Riktig. Svaret stemmer med beregnet verdi.",
    };
  }

  if (delta <= 0.05) {
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

function buildThreadApprenticeGuide({
  type,
  sizeLabel,
  majorDiameterMm,
  pitchLabel,
  pitchMm,
  drillDiameterMm,
  threadDepthMm,
}: {
  type: ThreadType;
  sizeLabel: string | undefined;
  majorDiameterMm: number | undefined;
  pitchLabel: string | undefined;
  pitchMm: number | undefined;
  drillDiameterMm: number | undefined;
  threadDepthMm: number | undefined;
}): ThreadApprenticeGuide | null {
  if (
    !sizeLabel ||
    !pitchLabel ||
    majorDiameterMm === undefined ||
    pitchMm === undefined ||
    drillDiameterMm === undefined ||
    threadDepthMm === undefined
  ) {
    return null;
  }

  const depthFactor = type === "bsp" ? 0.640327 : 0.541266;
  const drillFormula = type === "bsp" ? "practical table value" : "D - P";
  const drillCalculation =
    type === "bsp"
      ? "selected value ="
      : `${formatGuideNumber(majorDiameterMm)} - ${formatGuideNumber(
          pitchMm,
        )} =`;

  return {
    sizeLabel,
    pitchLabel,
    drillDiameterMm,
    threadDepthMm,
    drillFormula,
    drillCalculation,
    depthFormula: "factor x P",
    depthCalculation: `${formatGuideNumber(depthFactor, 6)} x ${formatGuideNumber(
      pitchMm,
    )} =`,
  };
}

function formatGuideNumber(value: number, decimals = 3) {
  return value.toFixed(decimals);
}

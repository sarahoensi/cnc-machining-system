import type {
  ThreadApiType,
  ThreadOptionsResponse,
  ThreadPitchOption,
  ThreadSizeOption,
  ThreadType,
} from "../api/types";
import { userField } from "@shared/form/types/fields";
import type { ThreadFormState } from "./threadForm";

export const threadTypeOptions = [
  { value: "metric", label: "Metric" },
  { value: "unified", label: "UNF/UNC" },
  { value: "bsp", label: "G/BSP" },
] as const;

export const emptyThreadOptions: ThreadOptionsResponse = {
  types: [],
  metric: [],
  unc: [],
  unf: [],
  bsp: [],
};

export function getThreadSizes(options: ThreadOptionsResponse, type: ThreadType) {
  if (type === "unified") {
    return getUnifiedThreadSizes(options);
  }

  return options[type];
}

export function getThreadSize(
  options: ThreadOptionsResponse,
  type: ThreadType,
  value: string,
) {
  return getThreadSizes(options, type).find((option) => option.value === value);
}

export function getThreadPitch(
  options: ThreadOptionsResponse,
  type: ThreadType,
  size: string,
  value: string,
) {
  return getThreadSize(options, type, size)?.pitches.find(
    (option) => option.value === value,
  );
}

export function reconcileThreadSelection(
  form: ThreadFormState,
  options: ThreadOptionsResponse,
): ThreadFormState {
  const sizes = getThreadSizes(options, form.extras.type);
  const currentSize = sizes.find((option) => option.value === form.fields.size.value);
  const size = currentSize ?? sizes[0];
  const currentPitch = size?.pitches.find(
    (option) => option.value === form.fields.pitch.value,
  );
  const pitch = currentPitch ?? getDefaultThreadPitch(size);

  return {
    ...form,
    fields: {
      ...form.fields,
      size: userField(size?.value ?? ""),
      pitch: userField(pitch?.value ?? ""),
    },
  };
}

export function getDefaultThreadPitch(
  size: ThreadSizeOption | undefined,
) {
  return size?.pitches.find((option) => option.isDefaultPitch) ?? size?.pitches[0];
}

export function getPitchApiSelection(
  type: ThreadType,
  pitch: string,
): { type: ThreadApiType; pitch: string } | null {
  if (type !== "unified") {
    return { type, pitch };
  }

  const [sourceType, sourcePitch] = pitch.split(":", 2);
  if (!isUnifiedSourceType(sourceType) || !sourcePitch) {
    return null;
  }

  return { type: sourceType, pitch: sourcePitch };
}

function getUnifiedThreadSizes(options: ThreadOptionsResponse) {
  const byDesignation = new Map<string, ThreadSizeOption>();

  for (const [sourceType, sizes] of [
    ["unc", options.unc],
    ["unf", options.unf],
  ] as const) {
    for (const size of sizes) {
      const existing = byDesignation.get(size.value);
      const pitches = size.pitches.map((pitch) =>
        toUnifiedPitchOption(pitch, sourceType),
      );

      if (!existing) {
        byDesignation.set(size.value, {
          ...size,
          pitches,
        });
        continue;
      }

      existing.majorDiameterMm = Math.min(
        existing.majorDiameterMm,
        size.majorDiameterMm,
      );
      existing.pitches = [...existing.pitches, ...pitches].sort(
        compareUnifiedPitches,
      );
    }
  }

  return [...byDesignation.values()].sort((left, right) =>
    left.majorDiameterMm === right.majorDiameterMm
      ? left.label.localeCompare(right.label)
      : left.majorDiameterMm - right.majorDiameterMm,
  );
}

function toUnifiedPitchOption(
  pitch: ThreadPitchOption,
  sourceType: "unc" | "unf",
): ThreadPitchOption {
  return {
    ...pitch,
    value: `${sourceType}:${pitch.value}`,
    series: sourceType === "unc" ? "coarse" : "fine",
    sourceType,
  };
}

function compareUnifiedPitches(left: ThreadPitchOption, right: ThreadPitchOption) {
  const leftOrder = left.sourceType === "unc" ? 0 : 1;
  const rightOrder = right.sourceType === "unc" ? 0 : 1;

  return leftOrder === rightOrder
    ? right.pitchMm - left.pitchMm
    : leftOrder - rightOrder;
}

function isUnifiedSourceType(value: string): value is "unc" | "unf" {
  return value === "unc" || value === "unf";
}

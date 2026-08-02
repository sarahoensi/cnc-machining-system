/**
 * @vitest-environment jsdom
 */

import { DisplaySettingProvider } from "@app/providers/DisplaySettingProvider";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { userField } from "@shared/form/types/fields";

import {
  createInitialCuttingDataForm,
  type CuttingDataKey,
} from "./cuttingData/domain/cuttingDataForm";
import { CuttingDataForm } from "./cuttingData/ui/form/CuttingDataForm";
import type { useCuttingPageController } from "./cuttingData/ui/useCuttingPageController";

import { createInitialHelixForm, type HelixKey } from "./helix/domain/helixForm";
import { HelixForm } from "./helix/ui/form/HelixForm";
import type { useHelixPageController } from "./helix/ui/useHelixPageController";

import {
  createInitialTriangleForm,
  type TriangleKey,
} from "./right_triangle/domain/triangleForm";
import { TriangleForm } from "./right_triangle/ui/form/TriangleForm";
import type { useTrianglePageController } from "./right_triangle/ui/useTrianglePageController";

import {
  createInitialToleranceForm,
  type ToleranceKey,
} from "./tolerances/domain/toleranceForm";
import type { ToleranceObjectType } from "./tolerances/api/types";
import { TolerancesForm } from "./tolerances/ui/form/TolerancesForm";
import { getToleranceSelectState } from "./tolerances/ui/toleranceSelectState";
import type { useTolerancePageController } from "./tolerances/ui/useTolerancePageController";

import { createInitialThreadForm, type ThreadKey } from "./threads/domain/threadForm";
import { ThreadsForm } from "./threads/ui/form/ThreadsForm";
import type { useThreadsPageController } from "./threads/ui/useThreadsPageController";

type CuttingController = ReturnType<typeof useCuttingPageController>;
type HelixController = ReturnType<typeof useHelixPageController>;
type TriangleController = ReturnType<typeof useTrianglePageController>;
type ToleranceController = ReturnType<typeof useTolerancePageController>;
type ThreadsController = ReturnType<typeof useThreadsPageController>;

function createNavigationStub<K extends string>() {
  return {
    containerRef: { current: null },
    register: () => () => undefined,
    registerSubmitAction: () => undefined,
    handleKeyDown: () => () => undefined,
    handleSubmitActionKeyDown: () => undefined,
    onFieldFocus: vi.fn(),
    onFieldBlur: vi.fn(),
    focusFirstAfterRender: vi.fn(),
    focusAfterCalculate: vi.fn(),
    focusAfterReset: vi.fn(),
  } as unknown as {
    register: (key: K) => (element: Element | null) => void;
  };
}

function withDisplaySettings(children: React.ReactNode) {
  return <DisplaySettingProvider>{children}</DisplaySettingProvider>;
}

function TestCuttingDataForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
  onSave = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
  onSave?: () => void;
}) {
  const [form, setForm] = useState(createInitialCuttingDataForm);

  const controller = {
    form,
    navigation: createNavigationStub<CuttingDataKey>(),
    onFieldChange: (key: CuttingDataKey, value: string) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    calculate: async () => {
      onCalculate();
    },
    resetForm: onReset,
    history: [],
    save: onSave,
    load: vi.fn(),
    remove: vi.fn(),
    clear: vi.fn(),
  } as unknown as CuttingController;

  return withDisplaySettings(<CuttingDataForm controller={controller} />);
}

function TestHelixForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
}) {
  const [form, setForm] = useState(createInitialHelixForm);

  const controller = {
    form,
    activeField: null,
    navigation: createNavigationStub<HelixKey>(),
    onFieldChange: (key: HelixKey, value: string) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    onModeChange: (mode: "Inner" | "Outer") => {
      setForm((prev) => ({
        ...prev,
        extras: {
          ...prev.extras,
          mode,
        },
      }));
    },
    calculate: async () => {
      onCalculate();
    },
    resetForm: onReset,
  } as unknown as HelixController;

  return withDisplaySettings(<HelixForm controller={controller} />);
}

function TestTriangleForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
}) {
  const [form, setForm] = useState(createInitialTriangleForm);

  const controller = {
    form,
    activeField: null,
    navigation: createNavigationStub<TriangleKey>(),
    onFieldChange: (key: TriangleKey, value: string) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    calculate: async () => {
      onCalculate();
    },
    resetForm: onReset,
  } as unknown as TriangleController;

  return withDisplaySettings(<TriangleForm controller={controller} />);
}

function TestTolerancesForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
}) {
  const [form, setForm] = useState(() => {
    const initial = createInitialToleranceForm();
    return {
      ...initial,
      extras: {
        ...initial.extras,
        loadingOptions: false,
        options: {
          holes: [
            { feature: "hole" as const, zone: "H", grades: [7, 8] },
            { feature: "hole" as const, zone: "JS", grades: [6, 7] },
          ],
          shafts: [
            { feature: "shaft" as const, zone: "h", grades: [6, 7] },
            { feature: "shaft" as const, zone: "g", grades: [6, 7] },
          ],
        },
      },
    };
  });

  const selectState = getToleranceSelectState(form);
  const controller = {
    form,
    mode: form.extras.mode,
    loadingOptions: form.extras.loadingOptions,
    ...selectState,
    onModeChange: (mode: "hole" | "shaft") => {
      setForm((prev) => ({
        ...prev,
        extras: {
          ...prev.extras,
          mode,
        },
      }));
    },
    onFieldChange: (key: ToleranceKey, value: string) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    onToleranceLetterChange: (feature: ToleranceObjectType, value: string) => {
      const key = `${feature}_letter` as ToleranceKey;
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    onToleranceGradeChange: (feature: ToleranceObjectType, value: string) => {
      const key = `${feature}_grade` as ToleranceKey;
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    calculate: async () => {
      onCalculate();
      return form;
    },
    resetForm: onReset,
    history: [],
    save: vi.fn(),
    load: vi.fn(),
    remove: vi.fn(),
    clear: vi.fn(),
  } as unknown as ToleranceController;

  return withDisplaySettings(<TolerancesForm controller={controller} />);
}

function TestThreadsForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
  onSave = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
  onSave?: () => void;
}) {
  const [form, setForm] = useState(() => {
    const initial = createInitialThreadForm();
    return {
      ...initial,
      status: "solved" as const,
      fields: {
        ...initial.fields,
        size: userField("M10"),
        pitch: userField("1.5"),
      },
      extras: {
        ...initial.extras,
        loadingOptions: false,
        options: {
          types: [
            { value: "metric" as const, label: "Metric" },
            { value: "unc" as const, label: "UNC" },
          ],
          metric: [
            {
              value: "M10",
              label: "M10",
              majorDiameterMm: 10,
              pitches: [
                {
                  value: "1.5",
                  label: "1.5 mm",
                  pitchMm: 1.5,
                  series: "coarse",
                  isDefaultPitch: true,
                },
                {
                  value: "1.25",
                  label: "1.25 mm",
                  pitchMm: 1.25,
                  series: "fine",
                  isDefaultPitch: false,
                },
              ],
            },
            {
              value: "M12",
              label: "M12",
              majorDiameterMm: 12,
              pitches: [
                {
                  value: "1.75",
                  label: "1.75 mm",
                  pitchMm: 1.75,
                  series: "coarse",
                  isDefaultPitch: true,
                },
              ],
            },
          ],
          unc: [],
          unf: [],
          bsp: [],
        },
      },
    };
  });

  const typeOptions = [
    { value: "metric" as const, label: "Metric" },
    { value: "unified" as const, label: "UNF/UNC" },
    { value: "bsp" as const, label: "G/BSP" },
  ];
  const sizeOptions = form.extras.options.metric.map((option) => ({
    value: option.value,
    label: option.label,
  }));
  const selectedSize = form.extras.options.metric.find(
    (option) => option.value === form.fields.size.value,
  );
  const pitchOptions =
    selectedSize?.pitches.map((option) => ({
      value: option.value,
      label: option.label.replace(/\s*mm$/i, ""),
      meta: option.series === "coarse" ? "Coarse" : "Fine",
      pitchMm: option.pitchMm,
    })) ?? [];

  const controller = {
    form,
    navigation: createNavigationStub<ThreadKey>(),
    type: form.extras.type,
    loadingOptions: form.extras.loadingOptions,
    typeOptions,
    sizeOptions,
    pitchOptions,
    onTypeChange: vi.fn(),
    onSizeChange: (value: string) => {
      const size = form.extras.options.metric.find((option) => option.value === value);
      const pitch = size?.pitches.find((option) => option.isDefaultPitch);
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          size: userField(value),
          pitch: userField(pitch?.value ?? ""),
        },
      }));
    },
    onPitchChange: (value: string) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          pitch: userField(value),
        },
      }));
    },
    onFieldChange: vi.fn(),
    calculate: async () => {
      onCalculate();
    },
    resetForm: onReset,
    history: [],
    save: onSave,
    load: vi.fn(),
    remove: vi.fn(),
    clear: vi.fn(),
  } as unknown as ThreadsController;

  return withDisplaySettings(<ThreadsForm controller={controller} />);
}

describe("calculator forms", () => {
  it("lets the user fill cutting data fields and trigger actions", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();
    const onSave = vi.fn();

    render(
      <TestCuttingDataForm
        onCalculate={onCalculate}
        onReset={onReset}
        onSave={onSave}
      />,
    );

    await user.type(screen.getByLabelText("Tool diameter D"), "10");
    await user.type(screen.getByLabelText("Toothcount z"), "4");
    await user.type(screen.getByLabelText("Rotations n"), "1200");

    expect(screen.getByLabelText("Tool diameter D")).toHaveValue("10");
    expect(screen.getByLabelText("Toothcount z")).toHaveValue("4");
    expect(screen.getByLabelText("Rotations n")).toHaveValue("1200");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Save result" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onSave).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user change helix mode and enter helix inputs", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();

    render(<TestHelixForm onCalculate={onCalculate} onReset={onReset} />);

    expect(screen.getByRole("radio", { name: "Outer" })).toBeChecked();

    await user.click(screen.getByRole("radio", { name: "Inner" }));
    await user.type(screen.getByLabelText("Nominal diameter"), "50");
    await user.type(screen.getByLabelText("Tool diameter"), "10");
    await user.type(screen.getByLabelText("Pitch"), "2.5");

    expect(screen.getByRole("radio", { name: "Inner" })).toBeChecked();
    expect(screen.getByLabelText("Nominal diameter")).toHaveValue("50");
    expect(screen.getByLabelText("Tool diameter")).toHaveValue("10");
    expect(screen.getByLabelText("Pitch")).toHaveValue("2.5");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user enter right triangle inputs and trigger actions", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();

    render(<TestTriangleForm onCalculate={onCalculate} onReset={onReset} />);

    await user.type(screen.getByLabelText("Katet a"), "3");
    await user.type(screen.getByLabelText("Katet b"), "4");
    await user.type(screen.getByLabelText("Hypotenus c"), "5");

    expect(screen.getByLabelText("Katet a")).toHaveValue("3");
    expect(screen.getByLabelText("Katet b")).toHaveValue("4");
    expect(screen.getByLabelText("Hypotenus c")).toHaveValue("5");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user configure a tolerance lookup", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();

    render(<TestTolerancesForm onCalculate={onCalculate} onReset={onReset} />);

    await user.type(screen.getByLabelText("Nominal Ø"), "42");
    await user.click(screen.getByRole("radio", { name: "Shaft" }));

    expect(screen.getByRole("radio", { name: "Shaft" })).toBeChecked();

    await user.click(screen.getByRole("button", { name: "h" }));
    await user.click(screen.getByRole("option", { name: "g" }));
    await user.click(screen.getByRole("button", { name: "7" }));
    await user.click(screen.getByRole("option", { name: "6" }));

    expect(screen.getByLabelText("Nominal Ø")).toHaveValue("42");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user select thread size and pitch", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();
    const onSave = vi.fn();

    render(
      <TestThreadsForm onCalculate={onCalculate} onReset={onReset} onSave={onSave} />,
    );

    await user.click(screen.getByRole("button", { name: "M10" }));
    await user.click(screen.getByRole("option", { name: "M12" }));

    expect(screen.getByRole("button", { name: "M12" })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /1\.75/ }));
    await user.click(screen.getByRole("option", { name: /1\.75/ }));

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Save result" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onSave).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });
});

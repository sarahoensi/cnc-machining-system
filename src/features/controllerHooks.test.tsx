/**
 * @vitest-environment jsdom
 */

import { FormStateProvider } from "@app/providers/FormStateProvider";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { solveCuttingData } from "./cuttingData/api/solveCuttingData";
import { useCuttingPageController } from "./cuttingData/ui/useCuttingPageController";
import { solveHelix } from "./helix/api/solveHelix";
import { useHelixPageController } from "./helix/ui/useHelixPageController";
import { solveTriangle } from "./right_triangle/api/solveTriangle";
import { useTrianglePageController } from "./right_triangle/ui/useTrianglePageController";
import { listThreadOptionsApi } from "./threads/api/client";
import { solveThread } from "./threads/api/solveThread";
import type { ThreadOptionsResponse } from "./threads/api/types";
import { useThreadsPageController } from "./threads/ui/useThreadsPageController";
import { listIso286ToleranceOptionsApi } from "./tolerances/api/client";
import { solveTolerance } from "./tolerances/api/solveTolerance";
import type { ToleranceOptionsResponse } from "./tolerances/api/types";
import { useTolerancePageController } from "./tolerances/ui/useTolerancePageController";
import { generateFinishingPlan } from "./finishing/plan/api/generateFinishingPlan";
import { registerFinishingMeasurement } from "./finishing/execution/api/registerFinishingMeasurement";
import { useFinishingPageController } from "./finishing/page/useFinishingPageController";

vi.mock("./cuttingData/api/solveCuttingData", () => ({
  solveCuttingData: vi.fn(),
}));

vi.mock("./helix/api/solveHelix", () => ({
  solveHelix: vi.fn(),
}));

vi.mock("./right_triangle/api/solveTriangle", () => ({
  solveTriangle: vi.fn(),
}));

vi.mock("./threads/api/client", () => ({
  listThreadOptionsApi: vi.fn(),
}));

vi.mock("./threads/api/solveThread", () => ({
  solveThread: vi.fn(),
}));

vi.mock("./tolerances/api/client", () => ({
  listIso286ToleranceOptionsApi: vi.fn(),
}));

vi.mock("./tolerances/api/solveTolerance", () => ({
  solveTolerance: vi.fn(),
}));

vi.mock("./finishing/plan/api/generateFinishingPlan", () => ({
  generateFinishingPlan: vi.fn(),
}));

vi.mock("./finishing/execution/api/registerFinishingMeasurement", () => ({
  registerFinishingMeasurement: vi.fn(),
}));

const cuttingSolveMock = vi.mocked(solveCuttingData);
const helixSolveMock = vi.mocked(solveHelix);
const triangleSolveMock = vi.mocked(solveTriangle);
const listThreadOptionsMock = vi.mocked(listThreadOptionsApi);
const threadSolveMock = vi.mocked(solveThread);
const listToleranceOptionsMock = vi.mocked(listIso286ToleranceOptionsApi);
const toleranceSolveMock = vi.mocked(solveTolerance);
const generateFinishingPlanMock = vi.mocked(generateFinishingPlan);
const registerFinishingMeasurementMock = vi.mocked(registerFinishingMeasurement);

function wrapper({ children }: { children: ReactNode }) {
  return <FormStateProvider>{children}</FormStateProvider>;
}

const threadOptions: ThreadOptionsResponse = {
  types: [
    { value: "metric", label: "Metric" },
    { value: "unc", label: "UNC" },
    { value: "unf", label: "UNF" },
    { value: "bsp", label: "G/BSP" },
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
  ],
  unc: [
    {
      value: "1/4",
      label: "1/4",
      majorDiameterMm: 6.35,
      pitches: [
        {
          value: "20",
          label: "20 TPI",
          pitchMm: 1.27,
          series: "coarse",
          isDefaultPitch: true,
          sourceType: "unc",
        },
      ],
    },
  ],
  unf: [],
  bsp: [],
};

const toleranceOptions: ToleranceOptionsResponse = {
  holes: [
    { feature: "hole", zone: "H", grades: [7, 8] },
    { feature: "hole", zone: "JS", grades: [6, 7] },
  ],
  shafts: [
    { feature: "shaft", zone: "h", grades: [6, 7] },
    { feature: "shaft", zone: "g", grades: [6, 7] },
  ],
};

beforeEach(() => {
  vi.clearAllMocks();
  globalThis.requestAnimationFrame ??= (callback: FrameRequestCallback) => {
    callback(0);
    return 0;
  };
});

describe("calculator page controllers", () => {
  it("updates cutting data fields and stores solved machine output", async () => {
    cuttingSolveMock.mockResolvedValue({ cutting_speed: 125.66, feed_rate: 480 });

    const { result } = renderHook(() => useCuttingPageController(), { wrapper });

    act(() => {
      result.current.onFieldChange("diameter", "10");
      result.current.onFieldChange("rpm", "4000");
      result.current.onFieldChange("teeth", "4");
    });

    await act(async () => {
      await result.current.calculate();
    });

    expect(cuttingSolveMock).toHaveBeenCalledWith({
      diameter: 10,
      rpm: 4000,
      teeth: 4,
    });
    expect(result.current.form.status).toBe("solved");
    expect(result.current.form.fields.cutting_speed.machineValue).toBe(125.66);
    expect(result.current.form.fields.feed_rate.machineValue).toBe(480);
  });

  it("passes helix mode into calculation and clears it on reset", async () => {
    helixSolveMock.mockResolvedValue({ pitch: 2.5, angle: 12.5 });

    const { result } = renderHook(() => useHelixPageController(), { wrapper });

    act(() => {
      result.current.onModeChange("Inner");
      result.current.onFieldChange("diameter", "50");
      result.current.onFieldChange("tool_diameter", "10");
      result.current.onFieldChange("pitch", "2.5");
    });

    await act(async () => {
      await result.current.calculate();
    });

    expect(helixSolveMock).toHaveBeenCalledWith(
      { diameter: 50, tool_diameter: 10, pitch: 2.5 },
      "Inner",
    );
    expect(result.current.form.fields.angle.machineValue).toBe(12.5);

    act(() => {
      result.current.resetForm();
    });

    expect(result.current.form.extras.mode).toBe("Outer");
    expect(result.current.form.fields.diameter.value).toBe("");
  });

  it("solves right triangle values from controller state", async () => {
    triangleSolveMock.mockResolvedValue({ c: 5, alpha: 36.87, beta: 53.13 });

    const { result } = renderHook(() => useTrianglePageController(), { wrapper });

    act(() => {
      result.current.onFieldChange("a", "3");
      result.current.onFieldChange("b", "4");
    });

    await act(async () => {
      await result.current.calculate();
    });

    expect(triangleSolveMock).toHaveBeenCalledWith({ a: 3, b: 4 }, {});
    expect(result.current.form.status).toBe("solved");
    expect(result.current.form.fields.c.machineValue).toBe(5);
  });

  it("loads thread options, reconciles defaults, and solves selected thread", async () => {
    listThreadOptionsMock.mockResolvedValue(threadOptions);
    threadSolveMock.mockResolvedValue({ drill_diameter: 8.5, thread_depth: 0.92 });

    const { result } = renderHook(() => useThreadsPageController(), { wrapper });

    await waitFor(() => expect(result.current.loadingOptions).toBe(false));

    expect(result.current.sizeOptions).toEqual([{ value: "M10", label: "M10" }]);
    expect(result.current.pitchOptions[0]).toMatchObject({
      value: "1.5",
      label: "1.5",
      meta: "Coarse",
    });

    await act(async () => {
      await result.current.calculate();
    });

    expect(threadSolveMock).toHaveBeenCalledWith(
      {
        type: "metric",
        size: "M10",
        pitch: "1.5",
      },
      expect.objectContaining({ type: "metric" }),
    );
    expect(result.current.form.fields.drill_diameter.machineValue).toBe(8.5);
    expect(result.current.form.fields.thread_depth.machineValue).toBe(0.92);
  });

  it("loads tolerance options and calculates the active tolerance mode", async () => {
    listToleranceOptionsMock.mockResolvedValue(toleranceOptions);
    toleranceSolveMock.mockResolvedValue({
      upper_um: 0.012,
      lower_um: 0,
      min_mm: 42,
      max_mm: 42.012,
    });

    const { result } = renderHook(() => useTolerancePageController(), { wrapper });

    await waitFor(() => expect(result.current.loadingOptions).toBe(false));

    act(() => {
      result.current.onFieldChange("nominal", "42");
      result.current.onModeChange("shaft");
      result.current.onToleranceLetterChange("shaft", "g");
      result.current.onToleranceGradeChange("shaft", "6");
    });

    await act(async () => {
      await result.current.calculate();
    });

    expect(toleranceSolveMock).toHaveBeenCalledWith(
      {
        feature: "shaft",
        nominalMm: 42,
        code: "g6",
      },
      expect.objectContaining({ mode: "shaft" }),
    );
    expect(result.current.form.fields.max_mm.machineValue).toBe(42.012);
  });

  it("keeps generated finishing execution in controller state", async () => {
    generateFinishingPlanMock.mockResolvedValue({
      activeStep: 1,
      finished: false,
      steps: [
        {
          index: 1,
          startMm: 50,
          plannedDeltaMm: -0.5,
          plannedEndMm: 49.5,
          measurementMm: null,
        },
      ],
    });
    registerFinishingMeasurementMock.mockResolvedValue({
      activeStep: null,
      finished: true,
      steps: [
        {
          index: 1,
          startMm: 50,
          plannedDeltaMm: -0.5,
          plannedEndMm: 49.5,
          measurementMm: 49.48,
        },
      ],
    });

    const { result } = renderHook(() => useFinishingPageController(), { wrapper });

    act(() => {
      result.current.updateForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          start_diameter_mm: {
            ...prev.fields.start_diameter_mm,
            value: "50",
            source: "user",
          },
          target_diameter_mm: {
            ...prev.fields.target_diameter_mm,
            value: "49.5",
            source: "user",
          },
          cuts: {
            ...prev.fields.cuts,
            value: "1",
            source: "user",
          },
        },
      }));
    });

    await act(async () => {
      await result.current.generate();
    });

    expect(generateFinishingPlanMock).toHaveBeenCalledWith(
      {
        start_diameter_mm: 50,
        target_diameter_mm: 49.5,
        cuts: 1,
      },
      { mode: "Inner", planning: "ByCuts" },
    );
    expect(result.current.formReadOnly).toBe(true);
    expect(result.current.execution?.steps[0].data.expectedDiameter).toBe(49.5);

    await act(async () => {
      await result.current.registerMeasurement(1, 49.48);
    });

    expect(registerFinishingMeasurementMock).toHaveBeenCalledWith({
      step_number: 1,
      measurement_mm: 49.48,
    });
    expect(result.current.execution?.finished).toBe(true);
  });
});

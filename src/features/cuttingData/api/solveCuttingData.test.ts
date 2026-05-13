import { describe, expect, it, vi } from "vitest";

import { solveCuttingData } from "./solveCuttingData";

vi.mock("./client", () => ({
  solveCuttingDataApi: vi.fn(),
}));

import { solveCuttingDataApi } from "./client";

describe("solveCuttingData", () => {
  it("maps backend response keys into UI form keys", async () => {
    vi.mocked(solveCuttingDataApi).mockResolvedValue({
      cutting_speed_m_per_min: 120,
      rpm: 2400,
      feed_rate_mm_per_min: 300,
      chip_load_mm_per_tooth: 0.04,
    });

    const result = await solveCuttingData({ diameter: 10, teeth: 3 });

    expect(result).toEqual({
      cutting_speed: 120,
      rpm: 2400,
      feed_rate: 300,
      chip_load: 0.04,
    });
  });

  it("keeps output partial when backend omits values", async () => {
    vi.mocked(solveCuttingDataApi).mockResolvedValue({
      rpm: 1800,
    });

    const result = await solveCuttingData({ diameter: 8 });

    expect(result).toEqual({
      rpm: 1800,
    });
  });
});

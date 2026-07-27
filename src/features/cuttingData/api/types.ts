// features/cutting_data/api/types.ts

export type SolveCuttingDataRequest = Partial<{
  diameter_mm: number;
  teeth: number;
  cutting_speed_m_per_min: number;
  rpm: number;
  feed_rate_mm_per_min: number;
  chip_load_mm_per_tooth: number;
}>;

export type SolveCuttingDataResponse = {
  cutting_speed_m_per_min?: number;
  rpm?: number;
  chip_load_mm_per_tooth?: number;
  feed_rate_mm_per_min?: number;
};

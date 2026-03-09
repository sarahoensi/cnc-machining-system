// features/cutting_data/domain/cuttingDataConstraints.ts

import { CuttingDataKey } from "./cuttingDataForm";

export const validCuttingDataInputSets = [
  [
    "diameter",
    "teeth",
    "cutting_speed",
    "rpm",
    "feed_rate",
    "chip_load"
  ],
  [
    "diameter",
    "cutting_speed",
    "rpm",
    
  ],
  [
    "diameter",
    "teeth",
    "cutting_speed",
    "rpm",
  ],

] as const;

export const mutuallyExclusiveCuttingDataPairs: readonly (readonly [CuttingDataKey, CuttingDataKey])[] = [
  ["cutting_speed", "rpm"],
  ["feed_rate", "chip_load"]
] as const;
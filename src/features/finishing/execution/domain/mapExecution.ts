// features/finishing/domain/execution/mapExecution.ts

import { createExecutionState } from "@shared/execution/executionState"

import type {
  FinishingExecutionResponse
} from "../../api/types"

export type FinishingStepData = {
  startDiameter: number
  deltaD: number
  expectedDiameter: number
}

export function mapFinishingExecution(
  response: FinishingExecutionResponse
) {

  return createExecutionState(
    response.steps.map(s => ({

      index: s.index,

      measurement: s.measurementMm,

      data: {
        startDiameter: s.startMm,
        deltaD: s.plannedDeltaMm,
        expectedDiameter: s.plannedEndMm,
      }

    }))
  )
}
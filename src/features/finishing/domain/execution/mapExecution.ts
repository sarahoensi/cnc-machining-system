// features/domain/execution/mapExecution.ts

import {
  createExecutionState} from "@shared/execution/executionState"

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

      measurement: s.measurement_mm,

      data: {
        startDiameter: s.start_mm,
        deltaD: s.planned_delta_mm,
        expectedDiameter: s.planned_end_mm,
      }

    }))
  )
}
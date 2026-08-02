/**
 * @vitest-environment jsdom
 */

import { DisplaySettingProvider } from "@app/providers/DisplaySettingProvider";
import { createExecutionState } from "@shared/execution";
import { userField } from "@shared/form/types/fields";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import {
  createInitialFinishingForm,
  type FinishingFormState,
} from "./plan/domain/finishingForm";
import { PlanForm } from "./plan/ui/PlanForm";
import { FinishingExecutionTable } from "./execution/ui/ExecutionTable";

function TestPlanForm({
  onGenerate = vi.fn(),
  onReset = vi.fn(),
}: {
  onGenerate?: () => void;
  onReset?: () => void;
}) {
  const [form, setForm] = useState<FinishingFormState>(createInitialFinishingForm);

  function setControlledForm(
    value: FinishingFormState | ((prev: FinishingFormState) => FinishingFormState),
  ) {
    setForm((prev) => {
      const next = typeof value === "function" ? value(prev) : value;

      return {
        ...next,
        fields: Object.fromEntries(
          Object.entries(next.fields).map(([key, field]) => [
            key,
            field.source === "user" ? userField(field.value, field) : field,
          ]),
        ) as FinishingFormState["fields"],
      };
    });
  }

  return (
    <DisplaySettingProvider>
      <PlanForm
        form={form}
        setForm={setControlledForm}
        onGenerate={() => {
          onGenerate();
          return form;
        }}
        onReset={onReset}
        onEdit={vi.fn()}
        readOnly={false}
      />
    </DisplaySettingProvider>
  );
}

describe("finishing RTL flows", () => {
  it("lets the user configure and generate a finishing plan", async () => {
    const user = userEvent.setup();
    const onGenerate = vi.fn();
    const onReset = vi.fn();

    render(<TestPlanForm onGenerate={onGenerate} onReset={onReset} />);

    expect(screen.getByRole("radio", { name: "Inner" })).toBeChecked();

    await user.click(screen.getByRole("radio", { name: "Outer" }));
    await user.type(screen.getByLabelText("Start diameter"), "50");
    await user.type(screen.getByLabelText("Target diameter"), "49");
    await user.type(screen.getByLabelText("Number of cuts"), "2");

    expect(screen.getByRole("radio", { name: "Outer" })).toBeChecked();
    expect(screen.getByLabelText("Start diameter")).toHaveValue("50");
    expect(screen.getByLabelText("Target diameter")).toHaveValue("49");
    expect(screen.getByLabelText("Number of cuts")).toHaveValue("2");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onGenerate).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user register a measurement for the active finishing step", async () => {
    const user = userEvent.setup();
    const onRegisterMeasurement = vi.fn(async () => undefined);
    const execution = createExecutionState(
      [
        {
          index: 1,
          data: {
            startDiameter: 50,
            deltaD: -0.5,
            expectedDiameter: 49.5,
          },
        },
        {
          index: 2,
          data: {
            startDiameter: 49.5,
            deltaD: -0.5,
            expectedDiameter: 49,
          },
        },
      ],
      false,
    );

    render(
      <DisplaySettingProvider>
        <FinishingExecutionTable
          execution={execution}
          onRegisterMeasurement={onRegisterMeasurement}
        />
      </DisplaySettingProvider>,
    );

    const activeRow = screen.getByRole("row", {
      name: /1\s+50\.000\s+-0\.500/i,
    });
    const measurementInput = within(activeRow).getByPlaceholderText("49.500");

    await user.type(measurementInput, "49.48");
    await user.click(within(activeRow).getByRole("button", { name: "Registrer" }));

    expect(onRegisterMeasurement).toHaveBeenCalledWith(1, 49.48);
  });
});

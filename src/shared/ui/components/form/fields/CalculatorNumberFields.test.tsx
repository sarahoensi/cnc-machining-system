import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";

import { emptyField } from "@shared/form/types/fields";

vi.mock("./FormNumberField", () => ({
  FormNumberField: ({
    label,
    disabled,
  }: {
    label: string;
    disabled?: boolean;
  }) => <div data-disabled={disabled ? "true" : "false"}>{label}</div>,
}));

import { CalculatorNumberFields } from "./CalculatorNumberFields";

describe("CalculatorNumberFields", () => {
  it("renders configs in order and preserves disabled field rules", () => {
    const markup = renderToStaticMarkup(
      <CalculatorNumberFields
        configs={[
          { key: "first", label: "First field" },
          { key: "second", label: "Second field", readOnly: true },
        ]}
        fields={{
          first: emptyField({ locked: true }),
          second: emptyField(),
        }}
        onChange={() => undefined}
      />,
    );

    expect(markup.indexOf("First field")).toBeLessThan(
      markup.indexOf("Second field"),
    );
    expect(markup.match(/data-disabled="true"/g)).toHaveLength(2);
  });
});

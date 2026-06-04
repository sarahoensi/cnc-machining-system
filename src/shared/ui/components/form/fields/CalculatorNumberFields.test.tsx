import { renderToStaticMarkup } from "react-dom/server";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { emptyField } from "@shared/form/types/fields";

const { renderedFields } = vi.hoisted(() => ({
  renderedFields: [] as Array<{
    label: string;
    disabled?: boolean;
    onChange: (value: string) => void;
    onFocus?: () => void;
    onBlur?: () => void;
  }>,
}));

vi.mock("./FormNumberField", () => ({
  FormNumberField: (props: {
    label: string;
    disabled?: boolean;
    onChange: (value: string) => void;
    onFocus?: () => void;
    onBlur?: () => void;
  }) => {
    renderedFields.push(props);
    return (
      <div data-disabled={props.disabled ? "true" : "false"}>
        {props.label}
      </div>
    );
  },
}));

import { CalculatorNumberFields } from "./CalculatorNumberFields";

describe("CalculatorNumberFields", () => {
  beforeEach(() => {
    renderedFields.length = 0;
  });

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

  it("forwards the field key to change and active-field callbacks", () => {
    const onChange = vi.fn();
    const onFocus = vi.fn();
    const onBlur = vi.fn();

    renderToStaticMarkup(
      <CalculatorNumberFields
        configs={[
          { key: "first", label: "First field" },
          { key: "second", label: "Second field" },
        ]}
        fields={{
          first: emptyField(),
          second: emptyField(),
        }}
        onChange={onChange}
        onFocus={onFocus}
        onBlur={onBlur}
      />,
    );

    renderedFields[1].onChange("42");
    renderedFields[1].onFocus?.();
    renderedFields[1].onBlur?.();

    expect(onChange).toHaveBeenCalledWith("second", "42");
    expect(onFocus).toHaveBeenCalledWith("second");
    expect(onBlur).toHaveBeenCalledWith("second");
  });
});

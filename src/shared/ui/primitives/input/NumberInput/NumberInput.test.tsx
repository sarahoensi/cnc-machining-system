import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { NumberInput } from "./NumberInput";

describe("NumberInput", () => {
  it("renders decimal input attributes and unit text", () => {
    const markup = renderToStaticMarkup(
      <NumberInput id="feed" value="12.5" unit="mm" onChange={() => undefined} />,
    );

    expect(markup).toContain('inputMode="decimal"');
    expect(markup).toContain('pattern="-?[0-9]*[.,]?[0-9]*"');
    expect(markup).toContain("12.5");
    expect(markup).toContain("mm");
  });

  it("renders source and disabled modifiers", () => {
    const markup = renderToStaticMarkup(
      <NumberInput value="1200" source="machine" disabled />,
    );

    expect(markup).toContain("input-control--machine");
    expect(markup).toContain("input-control--disabled");
    expect(markup).toContain("disabled");
  });
});

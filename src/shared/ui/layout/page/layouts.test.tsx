import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { FormLayout } from "../container/FormLayout/FormLayout";
import { FormSidebarLayout } from "./FormSidebarLayout/FormSidebarLayout";
import { SingleFormLayout } from "./SingleFormLayout/SingleFormLayout";

describe("form page layouts", () => {
  it("renders fields and actions in the standard form layout", () => {
    const markup = renderToStaticMarkup(
      <FormLayout
        fields={<div>Form fields</div>}
        actions={<button type="button">Calculate</button>}
      />,
    );

    expect(markup).toContain("Form fields");
    expect(markup).toContain("Calculate");
    expect(markup).toContain('class="form-layout-actions-slot"');
  });

  it("renders form and aside in the sidebar layout", () => {
    const markup = renderToStaticMarkup(
      <FormSidebarLayout
        form={<div>Form content</div>}
        sidebar={<div>Aside content</div>}
      />,
    );

    expect(markup).toContain("Form content");
    expect(markup).toContain("Aside content");
    expect(markup).toContain("<aside");
  });

  it("renders a single form without a figure slot", () => {
    const markup = renderToStaticMarkup(
      <SingleFormLayout form={<div>Single form</div>} />,
    );

    expect(markup).toContain("Single form");
    expect(markup).toContain('class="single-form-panel"');
    expect(markup).not.toContain("figure-panel");
  });
});

import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { FormLayout } from "../container/FormLayout/FormLayout";
import { SplitFormLayout } from "../container/SplitFormLayout/SplitFormLayout";
import { FormFigureLayout } from "./FormFigureLayout/FormFigureLayout";
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

  it("renders actions in the split form actions slot", () => {
    const markup = renderToStaticMarkup(
      <SplitFormLayout
        input={<div>Input fields</div>}
        output={<div>Output fields</div>}
        actions={<button type="button">Calculate</button>}
      />,
    );

    expect(markup).toContain("Calculate");
    expect(markup).toContain('class="split-form-actions"');
  });

  it("renders form and aside in the sidebar layout", () => {
    const markup = renderToStaticMarkup(
      <FormSidebarLayout
        formWidth="lg"
        form={<div>Form content</div>}
        sidebar={<div>Aside content</div>}
      />,
    );

    expect(markup).toContain("Form content");
    expect(markup).toContain("Aside content");
    expect(markup).toContain("<aside");
    expect(markup).toContain("form-width--lg");
  });

  it("renders a width variant on the figure layout", () => {
    const markup = renderToStaticMarkup(
      <FormFigureLayout
        form={<div>Form content</div>}
        figure={<div>Figure content</div>}
        formWidth="fluid"
      />,
    );

    expect(markup).toContain("form-width--fluid");
  });

  it("renders a single form without a figure slot", () => {
    const markup = renderToStaticMarkup(
      <SingleFormLayout form={<div>Single form</div>} formWidth="md" />,
    );

    expect(markup).toContain("Single form");
    expect(markup).toContain('class="single-form-panel"');
    expect(markup).toContain("form-width--md");
    expect(markup).not.toContain("figure-panel");
  });
});

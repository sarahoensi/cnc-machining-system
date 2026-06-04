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

  it("renders bottom actions with an explicit form layout modifier", () => {
    const markup = renderToStaticMarkup(
      <FormLayout
        fields={<div>Form fields</div>}
        actions={<button type="button">Calculate</button>}
        actionsPlacement="bottom"
      />,
    );

    expect(markup).toContain("form-layout--actions-bottom");
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

  it("renders explicit split form column and gap values", () => {
    const markup = renderToStaticMarkup(
      <SplitFormLayout
        input={<div>Input fields</div>}
        output={<div>Output fields</div>}
        actions={<button type="button">Calculate</button>}
        inputWidth="8rem"
        outputWidth="7.5rem"
        gap="var(--space-3)"
      />,
    );

    expect(markup).toContain("--split-form-input-width:8rem");
    expect(markup).toContain("--split-form-output-width:7.5rem");
    expect(markup).toContain("--split-form-gap:var(--space-3)");
  });

  it("renders form and aside in the sidebar layout", () => {
    const markup = renderToStaticMarkup(
      <FormSidebarLayout
        formWidth="lg"
        fillHeight
        form={<div>Form content</div>}
        sidebar={<div>Aside content</div>}
      />,
    );

    expect(markup).toContain("Form content");
    expect(markup).toContain("Aside content");
    expect(markup).toContain("<aside");
    expect(markup).toContain("form-width--lg");
    expect(markup).toContain("form-sidebar-layout--fill-height");
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

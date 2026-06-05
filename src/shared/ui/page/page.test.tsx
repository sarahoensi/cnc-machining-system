import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { Row } from "../primitives/Row/Row";
import { Split } from "../primitives/Split/Split";
import { Stack } from "../primitives/Stack/Stack";
import { FormActions } from "../form/FormActions";
import { FormLayout } from "../form/FormLayout";
import { FormPage } from "./FormPage";
import { PageShell } from "./PageShell";

describe("form page layouts", () => {
  it("renders fields and actions in the standard form layout", () => {
    const markup = renderToStaticMarkup(
      <FormLayout
        actions={<button type="button">Calculate</button>}
      >
        <div>Form fields</div>
      </FormLayout>,
    );

    expect(markup).toContain("Form fields");
    expect(markup).toContain("Calculate");
    expect(markup).toContain('class="form-layout-actions-slot"');
  });

  it("renders bottom actions with an explicit form layout modifier", () => {
    const markup = renderToStaticMarkup(
      <FormLayout
        actions={<button type="button">Calculate</button>}
        actionsPlacement="bottom"
      >
        <div>Form fields</div>
      </FormLayout>,
    );

    expect(markup).toContain("form-layout--actions-bottom");
  });

  it("renders primary and secondary slots in a split layout", () => {
    const markup = renderToStaticMarkup(
      <Split
        primary={<div>Primary content</div>}
        secondary={<div>Secondary content</div>}
      />,
    );

    expect(markup).toContain("Primary content");
    expect(markup).toContain("Secondary content");
    expect(markup).toContain('class="split-primary"');
    expect(markup).toContain('class="split-secondary"');
  });

  it("renders explicit split layout sizing and fill-height modifiers", () => {
    const markup = renderToStaticMarkup(
      <PageShell>
        <Split
          primaryWidth="19rem"
          primary={<div>Primary</div>}
          secondary={<div>Secondary</div>}
          fillHeight
          align="stretch"
          secondaryWidth="minmax(20rem, 1fr)"
          secondaryMinHeightOnCollapse="20rem"
          gap="var(--space-3)"
        />
      </PageShell>,
    );

    expect(markup).toContain("split--fill-height");
    expect(markup).toContain("split--align-stretch");
    expect(markup).toContain("--split-primary-width:19rem");
    expect(markup).toContain("--split-secondary-width:minmax(20rem, 1fr)");
    expect(markup).toContain("--split-secondary-min-height-on-collapse:20rem");
    expect(markup).toContain("--split-gap:var(--space-3)");
  });

  it("renders a single form page with a bounded form width", () => {
    const markup = renderToStaticMarkup(
      <FormPage form={<div>Single form</div>} panelWidth="320px" />,
    );

    expect(markup).toContain("Single form");
    expect(markup).toContain('class="page-shell form-page"');
    expect(markup).toContain('class="form-page-panel"');
    expect(markup).toContain("--form-page-panel-width:320px");
  });

  it("renders a responsive row with the requested column count", () => {
    const markup = renderToStaticMarkup(
      <Row columns={3}>
        <div>First</div>
        <div>Second</div>
        <div>Third</div>
      </Row>,
    );

    expect(markup).toContain("row--columns-3");
    expect(markup).toContain("Third");
  });

  it("renders stack children in order", () => {
    const markup = renderToStaticMarkup(
      <Stack>
        <div>First block</div>
        <div>Second block</div>
      </Stack>,
    );

    expect(markup).toContain("First block");
    expect(markup).toContain("Second block");
    expect(markup).toContain('class="stack"');
  });

  it("renders form actions with an inline modifier", () => {
    const markup = renderToStaticMarkup(
      <FormActions
        variant="inline"
        onCalculate={() => undefined}
        onReset={() => undefined}
      >
        <button type="button">Save</button>
      </FormActions>,
    );

    expect(markup).toContain("form-actions--inline");
    expect(markup).toContain("Calculate");
    expect(markup).toContain("Save");
    expect(markup).toContain("Clear form");
  });
});

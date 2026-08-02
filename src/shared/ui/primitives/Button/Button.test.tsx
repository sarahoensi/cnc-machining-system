/**
 * @vitest-environment jsdom
 */

import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import { Button } from "./Button";

describe("Button", () => {
  it("renders an accessible button and handles clicks", async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();

    render(<Button onClick={onClick}>Calculate</Button>);

    const button = screen.getByRole("button", { name: "Calculate" });

    expect(button).toBeInTheDocument();

    await user.click(button);

    expect(onClick).toHaveBeenCalledTimes(1);
  });
});

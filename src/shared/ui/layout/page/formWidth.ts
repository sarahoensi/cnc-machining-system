/**
 * Form-column width presets shared by page layouts.
 *
 * - `sm`: standard calculator form width (200px).
 * - `md`: wider single form that remains bounded by its container.
 * - `lg`: responsive wide form used for dense split-form content.
 * - `fluid`: lets the form column consume the available layout space.
 */
export type FormWidth = "sm" | "md" | "lg" | "fluid";

export function formWidthClassName(formWidth: FormWidth) {
  return `form-width--${formWidth}`;
}

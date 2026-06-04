export type FormWidth = "sm" | "md" | "lg" | "fluid";

export function formWidthClassName(formWidth: FormWidth) {
  return `form-width--${formWidth}`;
}

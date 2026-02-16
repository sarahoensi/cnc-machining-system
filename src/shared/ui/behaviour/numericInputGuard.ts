// shared/ui/input/numericInputGuard.ts

/**
 * Prevents invalid numeric key input.
 * Allows:
 * - digits
 * - one decimal separator ("," or ".")
 * - leading "-"
 */
export function handleNumericKeyDown(
  e: React.KeyboardEvent<HTMLInputElement>
) {
  const input = e.currentTarget;
  const key = e.key;

  const control = [
    "Backspace",
    "Delete",
    "ArrowLeft",
    "ArrowRight",
    "Tab",
    "Home",
    "End",
  ];

  if (control.includes(key)) return;

  const isDigit = /^\d$/.test(key);
  const isDecimal = key === "." || key === ",";
  const isMinus = key === "-";

  if (isDigit) return;

  if (isDecimal) {
    if (input.value.includes(".") || input.value.includes(",")) {
      e.preventDefault();
    }
    return;
  }

  if (isMinus) {
    if (input.selectionStart !== 0 || input.value.includes("-")) {
      e.preventDefault();
    }
    return;
  }

  e.preventDefault();
}

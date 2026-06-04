import type {
  KeyboardEventHandler,
  Ref,
} from "react";

import type { FieldState } from "@shared/form/types/fields";
import { FormNumberField } from "./FormNumberField";

export type CalculatorNumberFieldConfig<K extends string> = {
  key: K;
  label: string;
  tooltip?: string;
  unit?: string;
  autoFocus?: boolean;
  /**
   * Rendered as a disabled result field by this calculator-specific helper.
   * TODO: Rename to `disabled` or `isDisabled` when feature configs are standardized.
   */
  readOnly?: boolean;
};

type Props<K extends string> = {
  configs: readonly CalculatorNumberFieldConfig<K>[];
  fields: Record<K, FieldState>;
  onChange: (key: K, value: string) => void;
  register?: (key: K) => Ref<HTMLInputElement>;
  onKeyDown?: (key: K) => KeyboardEventHandler<HTMLInputElement>;
  onFocus?: (key: K) => void;
  onBlur?: (key: K) => void;
};

export function CalculatorNumberFields<K extends string>({
  configs,
  fields,
  onChange,
  register,
  onKeyDown,
  onFocus,
  onBlur,
}: Props<K>) {
  return configs.map((config) => {
    const fieldState = fields[config.key];

    return (
      <FormNumberField
        key={config.key}
        label={config.label}
        tooltip={config.tooltip}
        unit={config.unit}
        field={fieldState}
        disabled={fieldState.locked || config.readOnly}
        autoFocus={config.autoFocus}
        onChange={(value) => onChange(config.key, value)}
        ref={register?.(config.key)}
        onKeyDown={onKeyDown?.(config.key)}
        onFocus={onFocus ? () => onFocus(config.key) : undefined}
        onBlur={onBlur ? () => onBlur(config.key) : undefined}
      />
    );
  });
}

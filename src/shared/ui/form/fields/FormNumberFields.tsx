import type React from "react";

import type { FieldState } from "@shared/form/types/fields";

import { FormNumberField } from "./FormNumberField";

type NumberFieldConfig<K extends string> = {
  key: K;
  label: string;
  tooltip?: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

type Props<K extends string> = {
  configs: readonly NumberFieldConfig<K>[];
  fields: Record<K, FieldState>;
  onChange: (key: K, value: string) => void;
  register?: (key: K) => React.Ref<HTMLInputElement>;
  onKeyDown?: (key: K) => React.KeyboardEventHandler<HTMLInputElement>;
  onFocusField?: (key: K) => void;
  onBlurFields?: () => void;
};

export function FormNumberFields<K extends string>({
  configs,
  fields,
  onChange,
  register,
  onKeyDown,
  onFocusField,
  onBlurFields,
}: Props<K>) {
  return (
    <>
      {configs.map((config) => {
        const field = fields[config.key];

        return (
          <FormNumberField
            key={config.key}
            label={config.label}
            tooltip={config.tooltip}
            unit={config.unit}
            field={field}
            disabled={field.locked || config.readOnly}
            readonly={config.readOnly}
            autoFocus={config.autoFocus}
            onChange={(value) => onChange(config.key, value)}
            ref={register?.(config.key)}
            onKeyDown={onKeyDown?.(config.key)}
            onFocus={onFocusField ? () => onFocusField(config.key) : undefined}
            onBlur={onBlurFields}
          />
        );
      })}
    </>
  );
}

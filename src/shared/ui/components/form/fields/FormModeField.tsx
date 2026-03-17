// shared/ui/components/form/fields/FormModeField.tsx

import { Field } from "../Field/Field";
import { RadioGroup, RadioOption } from "../../primitives/RadioGroup/RadioGroup";

import "./FormModeField.css";

type Props<T extends string> = {
  label: string;
  value: T;
  onChange: (value: T) => void;
  options: readonly RadioOption<T>[];

  tooltip?: string;
  disabled?: boolean;
  readonly?: boolean;
};

export function FormModeField<T extends string>({
  label,
  tooltip,
  value,
  onChange,
  options,
  disabled,
  readonly,
}: Props<T>) {

  return (
    <Field
      label={label}
      tooltip={tooltip}
    >
      <RadioGroup
        name="mode"
        value={value}
        onChange={onChange}
        options={options}
        disabled={disabled}
        readonly={readonly}
      />
    </Field>
  );
}
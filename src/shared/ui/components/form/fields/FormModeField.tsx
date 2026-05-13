// shared/ui/components/form/fields/FormModeField.tsx

import { Field } from "../Field/Field";
import { RadioGroup, RadioOption } from "../../../primitives/RadioGroup/RadioGroup";

type Props<T extends string> = {
  label: string;
  value: T;
  onChange: (value: T) => void;
  options: readonly RadioOption<T>[];

  tooltip?: string;
  disabled?: boolean;
};

export function FormModeField<T extends string>({
  label,
  tooltip,
  value,
  onChange,
  options,
  disabled,
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
        className="radio-group--form"
      />
    </Field>
  );
}

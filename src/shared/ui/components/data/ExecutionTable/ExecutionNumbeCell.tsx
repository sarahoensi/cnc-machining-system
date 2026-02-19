// shared/ui/components/data/ExecutionTable/ExecutionNumberCell.tsx

import { ExecutionCell } from "./ExecutionCell";
import { NumberInput } from "@shared/ui/components/primitives/NumberInput/NumberInput";
import type { FieldState } from "@shared/types/fields";

type Props = {
  field: FieldState;
  onChange: (value: string) => void;
  unit?: string;
  disabled?: boolean;
  readonly?: boolean;
};

export function ExecutionNumberCell({
  field,
  onChange,
  unit,
  disabled,
  readonly = true,
}: Props) {
  return (
    <ExecutionCell align="right">
      <NumberInput
        field={field}
        onChange={onChange}
        unit={unit}
        disabled={disabled}
        readonly={readonly}
      />
    </ExecutionCell>
  );
}

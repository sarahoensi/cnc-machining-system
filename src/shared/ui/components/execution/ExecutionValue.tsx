// shared/ui/components/execution/ExecutionValue.tsx

import { NumberInput } from "@shared/ui/primitives/input";
import "./ExecutionField.css";

type Props = {
  value: string;
  unit?: string;
};

export function ExecutionValue({
  value,
  unit,
}: Props) {
  return (
    <div className="exec-field">
      <NumberInput
        value={value ?? ""}
        unit={unit}
        appearance="execution"
        displayOnly
      />
    </div>
  );
}

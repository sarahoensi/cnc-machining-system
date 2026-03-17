// shared/ui/components/execution/ExecutionInput.tsx

import { forwardRef } from "react";
import "./ExecutionField.css";
import { NumberInput } from "@shared/ui/components/primitives/NumberInput/NumberInput";

type Props = {
    value: string;
    placeholder?: string;
    unit?: string;
    error?: string;

    onChange: (value: string) => void;
    onSubmit?: () => void;
};

export const ExecutionInput = forwardRef<
    HTMLInputElement,
    Props
>(function ExecutionInput(
    {
        value,
        placeholder,
        unit,
        error,
        onChange,
        onSubmit,
    },
    ref
) {
    return (
        <div className="exec-field exec-field--input">
            <div className="exec-control">
                <NumberInput
                    ref={ref}
                    value={value}
                    placeholder={placeholder}
                    unit={unit}
                    onChange={onChange}
                    onKeyDown={(e) => {
                        if (e.key === "Enter") {
                            e.preventDefault();
                            onSubmit?.();
                        }
                    }}
                />
            </div>

            {error && (
                <div className="exec-error">
                    {error}
                </div>
            )}

        </div>
    );
});
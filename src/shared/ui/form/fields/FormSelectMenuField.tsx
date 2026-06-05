// src/shared/ui/components/form/fields/FormSelectMenuField.tsx

import {
  ForwardedRef,
  KeyboardEventHandler,
  ReactElement,
  ReactNode,
  RefAttributes,
  forwardRef,
  useEffect,
  useRef,
  useState,
} from "react";
import { Field } from "../Field";
import { SelectMenu } from "@shared/ui/primitives/Select";
import type {
  InputAppearance,
  InputSize,
  InputSource,
} from "@shared/ui/primitives/input";

type SelectOption<T extends string> = {
  value: T;
  label: ReactNode;
  meta?: ReactNode;
};

type UtilityItem = {
  label: ReactNode;
  onClick: () => void;
};

type Props<T extends string> = {
  label: string;
  tooltip?: string;
  valueLabel: ReactNode;
  options: SelectOption<T>[];
  onSelect: (value: T) => void;
  utilityItems?: UtilityItem[];
  className?: string;
  appearance?: InputAppearance;
  source?: InputSource;
  size?: InputSize;
  disabled?: boolean;
  onKeyDown?: KeyboardEventHandler<HTMLButtonElement>;
};

function FormSelectMenuFieldInner<T extends string>({
  label,
  tooltip,
  valueLabel,
  options,
  onSelect,
  utilityItems = [],
  className,
  appearance = "form",
  source = "default",
  size = "medium",
  disabled = false,
  onKeyDown,
}: Props<T>, ref: ForwardedRef<HTMLButtonElement>) {
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleOutsideClick(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handleOutsideClick);
    return () => document.removeEventListener("mousedown", handleOutsideClick);
  }, []);

  return (
    <Field label={label} tooltip={tooltip}>
      <div ref={menuRef}>
        <SelectMenu
          className={className}
          appearance={appearance}
          source={source}
          size={size}
          disabled={disabled}
          valueLabel={valueLabel}
          open={open}
          onToggle={() => setOpen((current) => !current)}
          triggerRef={ref}
          onKeyDown={onKeyDown}
          options={options}
          onSelect={(value) => {
            onSelect(value);
            setOpen(false);
          }}
          utilityItems={utilityItems.map((item) => ({
            label: item.label,
            onClick: () => {
              setOpen(false);
              item.onClick();
            },
          }))}
        />
      </div>
    </Field>
  );
}

export const FormSelectMenuField = forwardRef(FormSelectMenuFieldInner) as <
  T extends string,
>(
  props: Props<T> & RefAttributes<HTMLButtonElement>
) => ReactElement;


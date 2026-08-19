// src/shared/ui/primitives/Select/SelectMenu.tsx

import {
  KeyboardEvent,
  KeyboardEventHandler,
  ReactNode,
  Ref,
  useEffect,
  useId,
  useRef,
  useState,
} from "react";
import clsx from "clsx";
import type {
  InputAppearance,
  InputSize,
  InputSource,
} from "@shared/ui/primitives/input/types";
import "@shared/ui/primitives/input/InputControl/InputControl.css";
import "./SelectMenu.css";

type SelectOption<T extends string> = {
  value: T;
  label: ReactNode;
  meta?: ReactNode;
};

type SelectMenuLabelProps = {
  label: ReactNode;
  meta?: ReactNode;
  className?: string;
};

export function SelectMenuLabel({ label, meta, className }: SelectMenuLabelProps) {
  return (
    <span className={clsx("app-select-label", className)}>
      <span className="app-select-label-main">{label}</span>
      {meta ? <span className="app-select-label-meta">{meta}</span> : null}
    </span>
  );
}

type SelectMenuProps<T extends string> = {
  valueLabel: ReactNode;
  open: boolean;
  onToggle: () => void;
  options: SelectOption<T>[];
  onSelect: (value: T) => void;
  utilityItems?: { label: ReactNode; onClick: () => void }[];
  className?: string;
  appearance?: InputAppearance;
  source?: InputSource;
  size?: InputSize;
  disabled?: boolean;
  staticWhenSingleOption?: boolean;
  triggerRef?: Ref<HTMLButtonElement>;
  onKeyDown?: KeyboardEventHandler<HTMLButtonElement>;
};

export function SelectMenu<T extends string>({
  valueLabel,
  open,
  onToggle,
  options,
  onSelect,
  utilityItems = [],
  className,
  appearance = "form",
  source = "default",
  size = "medium",
  disabled = false,
  staticWhenSingleOption = false,
  triggerRef,
  onKeyDown,
}: SelectMenuProps<T>) {
  const id = useId();
  const dropdownId = `${id}-listbox`;
  const [activeIndex, setActiveIndex] = useState(0);
  const typeaheadBuffer = useRef("");
  const lastTypeaheadAt = useRef(0);
  const optionRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const optionCount = options.length;
  const isStaticSingleOption =
    staticWhenSingleOption && optionCount === 1 && utilityItems.length === 0;
  const isOpen = open && !disabled && !isStaticSingleOption;
  const activeOptionId =
    isOpen && optionCount > 0 ? getOptionId(dropdownId, activeIndex) : undefined;

  useEffect(() => {
    if (optionCount === 0) {
      setActiveIndex(0);
      return;
    }

    setActiveIndex((current) => Math.min(current, optionCount - 1));
  }, [optionCount]);

  useEffect(() => {
    if (!isOpen) {
      typeaheadBuffer.current = "";
      lastTypeaheadAt.current = 0;
    }
  }, [isOpen]);

  useEffect(() => {
    if (!isOpen) return;

    optionRefs.current[activeIndex]?.scrollIntoView({
      block: "nearest",
    });
  }, [activeIndex, isOpen]);

  function openMenu() {
    if (!open && !isStaticSingleOption) {
      onToggle();
    }
  }

  function closeMenu() {
    if (open) {
      onToggle();
    }
  }

  function selectActiveOption() {
    const option = options[activeIndex];
    if (option) {
      onSelect(option.value);
    }
  }

  function moveActiveOption(direction: 1 | -1) {
    if (optionCount === 0) return;

    setActiveIndex((current) => {
      const next = current + direction;
      if (next < 0) return optionCount - 1;
      if (next >= optionCount) return 0;
      return next;
    });
  }

  function findMatchingOption(search: string, startIndex: number) {
    if (!search) return -1;

    const normalizedSearch = normalizeSearchText(search);

    for (let offset = 1; offset <= optionCount; offset += 1) {
      const index = (startIndex + offset + optionCount) % optionCount;
      const option = options[index];
      const optionText = normalizeSearchText(getOptionSearchText(option));

      if (optionText.startsWith(normalizedSearch)) {
        return index;
      }
    }

    return -1;
  }

  function handleTypeahead(key: string) {
    const now = window.Date.now();
    const previousBuffer =
      now - lastTypeaheadAt.current <= TYPEAHEAD_RESET_MS
        ? typeaheadBuffer.current
        : "";
    const nextBuffer = `${previousBuffer}${key}`;
    const isRepeatedKeySearch =
      nextBuffer.length > 1 && [...nextBuffer].every((char) => char === key);
    const search = isRepeatedKeySearch ? key : nextBuffer;
    const startIndex = isRepeatedKeySearch ? activeIndex : -1;
    const matchIndex = findMatchingOption(search, startIndex);

    typeaheadBuffer.current = nextBuffer;
    lastTypeaheadAt.current = now;

    if (matchIndex !== -1) {
      setActiveIndex(matchIndex);
      openMenu();
    }
  }

  function handleTriggerKeyDown(event: KeyboardEvent<HTMLButtonElement>) {
    if (disabled) return;

    if (isStaticSingleOption) {
      onKeyDown?.(event);
      return;
    }

    if (!isOpen && (event.key === "ArrowDown" || event.key === "ArrowUp")) {
      onKeyDown?.(event);
      if (event.defaultPrevented) return;
    }

    if (event.key === "Enter") {
      event.preventDefault();
      if (isOpen) {
        selectActiveOption();
      } else {
        openMenu();
      }
      return;
    }

    if (event.key === " ") {
      event.preventDefault();
      if (isOpen) {
        selectActiveOption();
      } else {
        openMenu();
      }
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (!isOpen) {
        openMenu();
        return;
      }
      moveActiveOption(1);
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (!isOpen) {
        openMenu();
        return;
      }
      moveActiveOption(-1);
      return;
    }

    if (event.key === "Home" && isOpen) {
      event.preventDefault();
      setActiveIndex(0);
      return;
    }

    if (event.key === "End" && isOpen) {
      event.preventDefault();
      setActiveIndex(Math.max(0, optionCount - 1));
      return;
    }

    if (event.key === "Escape" && isOpen) {
      event.preventDefault();
      closeMenu();
      return;
    }

    if (event.key.length === 1 && !event.altKey && !event.ctrlKey && !event.metaKey) {
      event.preventDefault();
      handleTypeahead(event.key.toLocaleLowerCase());
    }
  }

  return (
    <div className={clsx("app-select-menu", isOpen && "is-open", className)}>
      <button
        type="button"
        ref={triggerRef}
        className={clsx(
          "app-select-trigger",
          "input-control",
          `input-control--${appearance}`,
          `input-control--${size}`,
          source !== "default" && `input-control--${source}`,
          disabled && "input-control--disabled",
          isStaticSingleOption && "app-select-trigger--static",
        )}
        onClick={() => {
          if (!disabled && !isStaticSingleOption) {
            onToggle();
          }
        }}
        onKeyDown={handleTriggerKeyDown}
        disabled={disabled}
        aria-haspopup={isStaticSingleOption ? undefined : "listbox"}
        aria-expanded={isStaticSingleOption ? undefined : isOpen}
        aria-controls={isStaticSingleOption ? undefined : dropdownId}
        aria-activedescendant={activeOptionId}
      >
        <span className="app-select-trigger-content">{valueLabel}</span>
        {isStaticSingleOption ? null : <span className="app-select-trigger-caret" />}
      </button>

      {isOpen ? (
        <div id={dropdownId} className="app-select-dropdown" role="listbox">
          {options.map((option, index) => (
            <button
              key={option.value}
              id={getOptionId(dropdownId, index)}
              type="button"
              ref={(element) => {
                optionRefs.current[index] = element;
              }}
              className={clsx(
                "app-select-option",
                index === activeIndex && "is-active",
              )}
              role="option"
              aria-selected={index === activeIndex}
              tabIndex={-1}
              onClick={() => onSelect(option.value)}
              onMouseEnter={() => setActiveIndex(index)}
            >
              <SelectMenuLabel
                className="app-select-option-content"
                label={option.label}
                meta={option.meta}
              />
            </button>
          ))}

          {utilityItems.length > 0 ? (
            <>
              <div className="app-select-divider" />
              {utilityItems.map((item, idx) => (
                <button
                  key={idx}
                  type="button"
                  className="app-select-utility"
                  onClick={item.onClick}
                >
                  {item.label}
                </button>
              ))}
            </>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

const TYPEAHEAD_RESET_MS = 600;

function getOptionId(dropdownId: string, index: number) {
  return `${dropdownId}-option-${index}`;
}

function getOptionSearchText<T extends string>(option: SelectOption<T>) {
  if (typeof option.label === "string" || typeof option.label === "number") {
    return String(option.label);
  }

  return option.value;
}

function normalizeSearchText(value: string) {
  return value.trim().toLocaleLowerCase();
}

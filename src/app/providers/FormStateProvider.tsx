import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";

type FormsStore = Record<string, unknown>;

type FormStateContextType = {
  forms: FormsStore;
  setForm: <T>(key: string, value: T | ((prev: T) => T)) => void;
};

const FormStateContext = createContext<FormStateContextType | null>(null);

export function resolveInitialForm<T>(
  form: T | undefined,
  cached: T | undefined,
  createInitial: () => T,
) {
  if (form !== undefined) {
    return { value: form, cache: cached, shouldHydrate: false };
  }

  const initial = cached ?? createInitial();
  return { value: initial, cache: initial, shouldHydrate: true };
}

export function FormStateProvider({ children }: { children: React.ReactNode }) {
  const [forms, setForms] = useState<FormsStore>({});

  const setForm = useCallback(<T,>(key: string, value: T | ((prev: T) => T)) => {
    setForms((prev) => {
      const prevValue = prev[key] as T;
      const nextValue =
        typeof value === "function" ? (value as (p: T) => T)(prevValue) : value;

      return {
        ...prev,
        [key]: nextValue,
      };
    });
  }, []);

  return (
    <FormStateContext.Provider value={{ forms, setForm }}>
      {children}
    </FormStateContext.Provider>
  );
}

export function useFeatureForm<T>(key: string, createInitial: () => T) {
  const ctx = useContext(FormStateContext);
  if (!ctx) {
    throw new Error("useFeatureForm must be used inside FormStateProvider");
  }

  const { forms, setForm } = ctx;
  const form = forms[key] as T | undefined;

  const cacheRef = useRef<T | undefined>(undefined);
  const resolved = resolveInitialForm(form, cacheRef.current, createInitial);
  cacheRef.current = resolved.cache;

  useEffect(() => {
    if (resolved.shouldHydrate) {
      setForm<T>(key, resolved.value);
    }
  }, [key, resolved.shouldHydrate, resolved.value, setForm]);

  const setFeatureForm = useCallback(
    (value: T | ((prev: T) => T)) => {
      setForm<T>(key, value);
    },
    [key, setForm],
  );

  return [resolved.value, setFeatureForm] as const;
}

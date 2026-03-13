// app/providers/FormStateProvider.tsx

import { createContext, useContext, useState } from "react";

type FormsStore = Record<string, unknown>;

type FormStateContextType = {
  forms: FormsStore;
  setForm: <T>(key: string, value: T | ((prev: T) => T)) => void;
};

const FormStateContext = createContext<FormStateContextType | null>(null);

export function FormStateProvider({ children }: { children: React.ReactNode }) {

  const [forms, setForms] = useState<FormsStore>({});

  function setForm<T>(key: string, value: T | ((prev: T) => T)) {
    setForms(prev => {
      const prevValue = prev[key] as T;

      const nextValue =
        typeof value === "function"
          ? (value as (p: T) => T)(prevValue)
          : value;

      return {
        ...prev,
        [key]: nextValue,
      };
    });
  }

  return (
    <FormStateContext.Provider value={{ forms, setForm }}>
      {children}
    </FormStateContext.Provider>
  );
}

export function useFeatureForm<T>(
  key: string,
  createInitial: () => T
) {
  const ctx = useContext(FormStateContext);
  if (!ctx) {
    throw new Error("useFeatureForm must be used inside FormStateProvider");
  }

  const { forms, setForm } = ctx;

  const form = forms[key] as T | undefined;

  if (form === undefined) {
    const initial = createInitial();
    setForm(key, initial);
    return [initial, (v: any) => setForm(key, v)] as const;
  }

  function setFeatureForm(value: T | ((prev: T) => T)) {
    setForm<T>(key, value);
  }

  return [form, setFeatureForm] as const;
}
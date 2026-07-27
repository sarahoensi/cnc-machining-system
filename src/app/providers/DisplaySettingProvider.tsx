// src/app/providers/DisplaySettingProvider.tsx

import { createContext, useContext, useEffect, useState } from "react";

/* ============================================================
   Types
============================================================ */

export type Decimals = 0 | 1 | 2 | 3 | 4 | 5 | 6;

type DisplaySettingContextValue = {
  decimals: Decimals;
  setDecimals: (value: Decimals) => void;
};

/* ============================================================
   Context
============================================================ */

const DisplaySettingContext = createContext<DisplaySettingContextValue | undefined>(
  undefined,
);

const STORAGE_KEY = "app-decimals";
const DEFAULT_DECIMALS: Decimals = 3;

/* ============================================================
   Helpers
============================================================ */

function getInitialDecimals(): Decimals {
  const stored = localStorage.getItem(STORAGE_KEY);

  if (!stored) return DEFAULT_DECIMALS;

  const parsed = Number(stored);

  if (Number.isInteger(parsed) && parsed >= 0 && parsed <= 6) {
    return parsed as Decimals;
  }

  return DEFAULT_DECIMALS;
}

/* ============================================================
   Provider
============================================================ */

export function DisplaySettingProvider({ children }: { children: React.ReactNode }) {
  const [decimals, setDecimalsState] = useState<Decimals>(getInitialDecimals);

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, String(decimals));
  }, [decimals]);

  function setDecimals(value: Decimals) {
    setDecimalsState(value);
  }

  return (
    <DisplaySettingContext.Provider
      value={{
        decimals,
        setDecimals,
      }}
    >
      {children}
    </DisplaySettingContext.Provider>
  );
}

/* ============================================================
   Hook
============================================================ */

export function useDisplaySettings() {
  const ctx = useContext(DisplaySettingContext);

  if (!ctx) {
    throw new Error("useDisplaySettings must be used inside DisplaySettingProvider");
  }

  return ctx;
}

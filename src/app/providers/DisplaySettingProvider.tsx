// src/app/providers/DisplaySettingProvider.tsx

import { createContext, useContext, useEffect, useState } from "react";

/* ============================================================
   Types
============================================================ */

export type Decimals = 0 | 1 | 2 | 3 | 4 | 5 | 6;

type DisplaySettingContextValue = {
  decimals: Decimals;
  setDecimals: (value: Decimals) => void;
  apprenticeMode: boolean;
  setApprenticeMode: (value: boolean) => void;
};

/* ============================================================
   Context
============================================================ */

const DisplaySettingContext = createContext<DisplaySettingContextValue | undefined>(
  undefined,
);

const DECIMALS_STORAGE_KEY = "app-decimals";
const APPRENTICE_MODE_STORAGE_KEY = "app-apprentice-mode";
const DEFAULT_DECIMALS: Decimals = 3;

/* ============================================================
   Helpers
============================================================ */

function getInitialDecimals(): Decimals {
  const stored = localStorage.getItem(DECIMALS_STORAGE_KEY);

  if (!stored) return DEFAULT_DECIMALS;

  const parsed = Number(stored);

  if (Number.isInteger(parsed) && parsed >= 0 && parsed <= 6) {
    return parsed as Decimals;
  }

  return DEFAULT_DECIMALS;
}

function getInitialApprenticeMode() {
  return localStorage.getItem(APPRENTICE_MODE_STORAGE_KEY) === "true";
}

/* ============================================================
   Provider
============================================================ */

export function DisplaySettingProvider({ children }: { children: React.ReactNode }) {
  const [decimals, setDecimalsState] = useState<Decimals>(getInitialDecimals);
  const [apprenticeMode, setApprenticeModeState] = useState(
    getInitialApprenticeMode,
  );

  useEffect(() => {
    localStorage.setItem(DECIMALS_STORAGE_KEY, String(decimals));
  }, [decimals]);

  useEffect(() => {
    localStorage.setItem(APPRENTICE_MODE_STORAGE_KEY, String(apprenticeMode));
  }, [apprenticeMode]);

  function setDecimals(value: Decimals) {
    setDecimalsState(value);
  }

  function setApprenticeMode(value: boolean) {
    setApprenticeModeState(value);
  }

  return (
    <DisplaySettingContext.Provider
      value={{
        decimals,
        setDecimals,
        apprenticeMode,
        setApprenticeMode,
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

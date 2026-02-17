import {
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";

/* ============================================================
   Types
============================================================ */

export type Theme =
  | "default"
  | "forest"
  | "pink"
  | "dark";

/* ============================================================
   Context
============================================================ */

type ThemeContextValue = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  toggleDark: () => void;
};

const ThemeContext =
  createContext<ThemeContextValue | undefined>(
    undefined
  );

const STORAGE_KEY = "app-theme";

/* ============================================================
   Helpers
============================================================ */

function getInitialTheme(): Theme {
  const stored = localStorage.getItem(
    STORAGE_KEY
  ) as Theme | null;

  if (stored) return stored;

  if (
    window.matchMedia(
      "(prefers-color-scheme: dark)"
    ).matches
  ) {
    return "dark";
  }

  return "default";
}

/* ============================================================
   Provider
============================================================ */

export function ThemeProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [theme, setTheme] =
    useState<Theme>(getInitialTheme);

  /* --------------------------------------------
     Persist + apply theme
  -------------------------------------------- */
  useEffect(() => {
    document.documentElement.setAttribute(
      "data-theme",
      theme
    );

    localStorage.setItem(STORAGE_KEY, theme);
  }, [theme]);

  /* --------------------------------------------
     Optional: listen to OS theme changes
  -------------------------------------------- */
  useEffect(() => {
    const media =
      window.matchMedia(
        "(prefers-color-scheme: dark)"
      );

    const handleChange = () => {
      const stored =
        localStorage.getItem(STORAGE_KEY);

      // only auto-switch if user has NOT chosen theme manually
      if (!stored) {
        setTheme(
          media.matches ? "dark" : "default"
        );
      }
    };

    media.addEventListener("change", handleChange);

    return () =>
      media.removeEventListener(
        "change",
        handleChange
      );
  }, []);

  function toggleDark() {
    setTheme((prev) =>
      prev === "dark" ? "default" : "dark"
    );
  }

  return (
    <ThemeContext.Provider
      value={{
        theme,
        setTheme,
        toggleDark,
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
}

/* ============================================================
   Hook
============================================================ */

export function useTheme() {
  const ctx = useContext(ThemeContext);

  if (!ctx) {
    throw new Error(
      "useTheme must be used inside ThemeProvider"
    );
  }

  return ctx;
}

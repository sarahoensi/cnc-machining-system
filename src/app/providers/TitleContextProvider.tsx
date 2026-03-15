// app/shell/TitleContextProvider.tsx

import {
  createContext,
  useContext,
  useState,
} from "react";

/* ============================================================
   Types
============================================================ */

type TitleContextValue = {
  title: string;
  setTitle: (title: string) => void;
};

/* ============================================================
   Context
============================================================ */

const TitleContext =
  createContext<TitleContextValue | undefined>(
    undefined
  );

/* ============================================================
   Provider
============================================================ */

export function TitleContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [title, setTitle] = useState("");

  return (
    <TitleContext.Provider
      value={{
        title,
        setTitle,
      }}
    >
      {children}
    </TitleContext.Provider>
  );
}

/* ============================================================
   Hooks
============================================================ */

export function useTitle() {
  const ctx = useContext(TitleContext);

  if (!ctx) {
    throw new Error(
      "useTitle must be used inside TitleContextProvider"
    );
  }

  return ctx;
}

/* Optional helper hook */

export function usePageTitle(title: string) {
  const { setTitle } = useTitle();

  useState(() => {
    setTitle(title);
  });
}
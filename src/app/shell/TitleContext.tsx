// app/shell/TitleContext.tsx

import { createContext, useContext, useState } from "react";

const TitleContext = createContext<{
  title: string;
  setTitle: (t: string) => void;
} | null>(null);

export function TitleProvider({ children }: { children: React.ReactNode }) {
  const [title, setTitle] = useState("");

  return (
    <TitleContext.Provider value={{ title, setTitle }}>
      {children}
    </TitleContext.Provider>
  );
}

export function useTitle() {
  const ctx = useContext(TitleContext);
  if (!ctx) throw new Error("useTitle must be used inside TitleProvider");
  return ctx;
}
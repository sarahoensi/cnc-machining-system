// app/shell/Page.tsx

import { useEffect } from "react";
import { useTitle } from "../providers/TitleContextProvider";

export function Page({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  const { setTitle } = useTitle();

  useEffect(() => {
    setTitle(title);
  }, [title, setTitle]);

  return <>{children}</>;
}
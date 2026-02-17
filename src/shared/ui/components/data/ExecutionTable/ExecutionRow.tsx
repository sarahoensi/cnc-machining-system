// ExecutionRow.tsx

import { ReactNode } from "react";

type Props = {
  children: ReactNode;
};

export function ExecutionRow({ children }: Props) {
  return <tr>{children}</tr>;
}

// app/App.tsx

import { AppShell } from "@app/shell/AppShell";
import { AppRoutes } from "./routes";

export default function App() {
  return (
    <AppShell>
      <AppRoutes />
    </AppShell>
  );
}

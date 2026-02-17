import { AppLayout } from "@app/shell/AppLayout";
import { AppRoutes } from "./routes";

export default function App() {
  return (
    <AppLayout>
      <AppRoutes />
    </AppLayout>
  );
}

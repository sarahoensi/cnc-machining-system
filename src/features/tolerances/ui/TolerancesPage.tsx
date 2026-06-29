// features/tolerances/ui/TolerancesPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { PageShell } from "@shared/ui/page/PageShell";
import { Stack } from "@shared/ui/primitives/Stack/Stack";

import { TolerancesForm } from "./form/TolerancesForm";
import { ToleranceHistoryPanel } from "./history/ToleranceHistoryPanel";
import { useTolerancePageController } from "./useTolerancePageController";
import "./TolerancesPage.css";

export function TolerancesPage() {
  usePageTitle("Tolerances");

  const controller = useTolerancePageController();

  return (
    <PageShell className="tolerances-page-layout">
      <Stack className="tolerances-page-stack">
        <div className="tolerances-page-form">
          <TolerancesForm controller={controller} />
        </div>

        <div className="tolerances-page-history">
          <ToleranceHistoryPanel
            history={controller.history}
            onLoad={controller.load}
            onDelete={controller.remove}
            onClear={controller.clear}
          />
        </div>
      </Stack>
    </PageShell>
  );
}

// features/cuttingData/ui/CuttingDataPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";

import { CuttingDataForm } from "./form/CuttingDataForm";
import { CuttingHistoryPanel } from "./history/CuttingHistoryPanel";
import { useCuttingPageController } from "./useCuttingPageController";

export function CuttingDataPage() {
  usePageTitle("Cutting Data");

  const controller = useCuttingPageController();

  return (
    <FormWithSidePanel
      fillHeight
      align="stretch"
      secondaryWidth="minmax(20rem, 1fr)"
      secondaryMinHeightOnCollapse="20rem"
      form={<CuttingDataForm controller={controller} />}
      sidePanel={
        <CuttingHistoryPanel
          history={controller.history}
          onLoad={controller.load}
          onDelete={controller.remove}
          onClear={controller.clear}
        />
      }
    />
  );
}

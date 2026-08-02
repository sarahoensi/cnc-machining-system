import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";

import { ThreadsForm } from "./form/ThreadsForm";
import { ThreadHistoryPanel } from "./history/ThreadHistoryPanel";
import { useThreadsPageController } from "./useThreadsPageController";

export function ThreadsPage() {
  usePageTitle("Thread Milling");

  const controller = useThreadsPageController();

  return (
    <FormWithSidePanel
      fillHeight
      align="stretch"
      secondaryWidth="minmax(20rem, 1fr)"
      secondaryMinHeightOnCollapse="20rem"
      form={<ThreadsForm controller={controller} />}
      sidePanel={
        <ThreadHistoryPanel
          history={controller.history}
          onLoad={controller.load}
          onDelete={controller.remove}
          onClear={controller.clear}
        />
      }
    />
  );
}

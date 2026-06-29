// features/helix/ui/HelixPage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";

import { HelixForm } from "./form/HelixForm";
import { HelixFigure } from "./helixFigure/HelixFigure";
import { useHelixPageController } from "./useHelixPageController";

export function HelixPage() {
  usePageTitle("Helix");

  const controller = useHelixPageController();

  return (
    <FormWithSidePanel
      form={<HelixForm controller={controller} />}
      sidePanel={
        <HelixFigure
          mode={controller.form.extras.mode}
          activeField={controller.activeField}
        />
      }
    />
  );
}

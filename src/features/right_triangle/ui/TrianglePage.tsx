// features/right_triangle/ui/TrianglePage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";

import { TriangleForm } from "./form/TriangleForm";
import { TriangleFigure } from "./triangleFigure/TriangleFigure";
import { useTrianglePageController } from "./useTrianglePageController";

export function TrianglePage() {
  usePageTitle("Triangle");

  const controller = useTrianglePageController();

  return (
    <FormWithSidePanel
      form={<TriangleForm controller={controller} />}
      sidePanel={<TriangleFigure activeField={controller.activeField} />}
    />
  );
}

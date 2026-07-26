// features/right_triangle/TrianglePage.tsx

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { FormWithSidePanel } from "@shared/ui/patterns/FormWithSidePanel/FormWithSidePanel";

import { TriangleForm } from "./ui/form/TriangleForm";
import { TriangleFigure } from "./ui/triangleFigure/TriangleFigure";
import { useTrianglePageController } from "./ui/useTrianglePageController";

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

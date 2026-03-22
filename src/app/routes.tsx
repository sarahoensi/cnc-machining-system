// src/app/routes.tsx

import { Routes, Route, Navigate } from "react-router-dom";

import { TrianglePage } from "@features/right_triangle/TrianglePage";
import { HelixPage } from "@features/helix/ui/HelixPage";
import  {CuttingDataPage} from "@features/cuttingData/ui/cuttingDataPage";
import { FinishingPage } from "@features/finishing/page/FinishingPage";


export function AppRoutes() {
  return (
    <Routes>
      {/* Default route */}
      <Route path="/" element={<Navigate to="/triangle" replace />} />

      {/* Geometry */}
      <Route path="/triangle" element={<TrianglePage />} />
      <Route path="/helix" element={<HelixPage />} />

      {/* Machining physics */}
      <Route path="/cutting" element={<CuttingDataPage />} />

      {/* Machining strategy */}
      <Route path="/finishing" element={<FinishingPage />} />

      {/* Fallback */}
      <Route path="*" element={<Navigate to="/triangle" replace />} />

    </Routes>
  );
}

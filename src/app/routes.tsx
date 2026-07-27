// src/app/routes.tsx

import { Routes, Route, Navigate } from "react-router-dom";

import { TrianglePage } from "@features/right_triangle/ui/TrianglePage";
import { HelixPage } from "@features/helix/ui/HelixPage";
import { CuttingDataPage } from "@features/cuttingData/ui/CuttingDataPage";
import { FinishingPage } from "@features/finishing/page/FinishingPage";
import { CylinderWeightPage } from "@features/cylinder_weight/ui/CylinderWeightPage";
import { TolerancesPage } from "@features/tolerances/ui/TolerancesPage";
import { ThreadsPage } from "@features/threads/ui/ThreadsPage";

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
      <Route path="/tolerances" element={<TolerancesPage />} />
      <Route path="/threads" element={<ThreadsPage />} />

      {/* Machining strategy */}
      <Route path="/finishing" element={<FinishingPage />} />
      <Route path="/cylinder-weight" element={<CylinderWeightPage />} />

      {/* Fallback */}
      <Route path="*" element={<Navigate to="/triangle" replace />} />
    </Routes>
  );
}

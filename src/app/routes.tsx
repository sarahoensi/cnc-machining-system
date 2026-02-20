// src/app/routes.tsx

import { Routes, Route, Navigate } from "react-router-dom";

import { PlaceholderPage } from "@app/PlaceholderPage";
import { TrianglePage } from "@features/right_triangle/TrianglePage";


export function AppRoutes() {
  return (
    <Routes>
      {/* Default route */}
      <Route path="/" element={<Navigate to="/triangle" replace />} />

      {/* Geometry */}
      <Route path="/triangle" element={<TrianglePage />} />
      <Route path="/helix" element={<PlaceholderPage title="Helix" />} />

      {/* Machining physics */}
      <Route path="/cutting" element={<PlaceholderPage title="Cutting Data" />} />

      {/* Machining strategy */}
      <Route path="/finishing" element={<PlaceholderPage title="Finishing" />} />

      {/* Fallback */}
      <Route path="*" element={<Navigate to="/triangle" replace />} />
    </Routes>
  );
}

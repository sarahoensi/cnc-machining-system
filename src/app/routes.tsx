// src/app/routes.tsx

import { Routes, Route, Navigate } from "react-router-dom";

import { PlaceholderPage } from "@app/PlaceholderPage";


export function AppRoutes() {
  return (
    <Routes>
      {/* Default route */}
      <Route path="/" element={<Navigate to="/triangle" replace />} />

      {/* Geometry */}
      <Route path="/triangle" element={<PlaceholderPage title="Right Triangle" />} />
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

// src/app/layout/AppLayout.tsx

import { useEffect, useState } from "react";
import { Topbar } from "./Topbar";
import { Sidebar } from "./Sidebar";
import "./AppLayout.css";

export function AppLayout({ children }: { children: React.ReactNode }) {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  useEffect(() => {
    const handleResize = () => {
      setIsSidebarOpen(window.innerWidth > 768);
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return (
    <div
      className={`app-layout ${
        isSidebarOpen ? "" : "sidebar-closed"
      }`}
    >
      <Topbar toggleSidebar={() => setIsSidebarOpen(prev => !prev)} />

      <div className="app-main">
        <aside className="sidebar-shell">
          <Sidebar />
        </aside>

        <main className="app-content">{children}</main>
      </div>
    </div>
  );
}

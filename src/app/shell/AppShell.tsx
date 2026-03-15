// src/app/layout/AppShell.tsx

import { useEffect, useState } from "react";
import { Topbar } from "../layout/Topbar";
import { Sidebar } from "../layout/Sidebar";
import "./AppShell.css";

export function AppShell({ children }: { children: React.ReactNode }) {
  const [sidebarOpen, setSidebarOpen] = useState(true);

  useEffect(() => {
    const handleResize = () => {
      setSidebarOpen(window.innerWidth > 768);
    };

    handleResize();
    window.addEventListener("resize", handleResize);

    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return (
    <div className={`app-shell ${sidebarOpen ? "" : "sidebar-collapsed"}`}>

      <header className="shell-topbar">
        <Topbar toggleSidebar={() => setSidebarOpen(v => !v)} />
      </header>

      <aside className="shell-sidebar">
        <Sidebar />
      </aside>

      <main className="shell-main">
        <div className="shell-content">
          {children}
        </div>

        <footer className="shell-footer">
          Footer
        </footer>
      </main>

    </div>
  );
}
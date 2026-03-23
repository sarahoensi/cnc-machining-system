import { Sidebar } from "@app/layout/Sidebar";
import { Topbar } from "@app/layout/Topbar";
import clsx from "clsx";
import { useEffect, useState } from "react";
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
    <div
      className={clsx(
        "app-shell",
        !sidebarOpen && "sidebar-collapsed"
      )}
    >
      <div className="shell-topbar">
        <Topbar toggleSidebar={() => setSidebarOpen(v => !v)} />
      </div>

      <aside className="shell-sidebar">
        <Sidebar />
      </aside>

      <main className="shell-main">
        <div className="shell-content">
          {children}
        </div>

        <footer className="shell-footer" />
      </main>
    </div>
  );
}
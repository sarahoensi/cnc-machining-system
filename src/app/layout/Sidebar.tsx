import { NavLink, useLocation } from "react-router-dom";
import "./Sidebar.css";

const NAV_ITEMS = [
  { to: "/triangle", label: "Triangle" },
  { to: "/helix", label: "Helix" },
  { to: "/cutting", label: "Cutting Data" },
  { to: "/tolerances", label: "Tolerances" },
  { to: "/cylinder-weight", label: "Cylinder Weight" },
  { to: "/finishing", label: "Finishing" },
];

export function Sidebar() {
  const location = useLocation();
  const isPathActive = (to: string) =>
    location.pathname === to || location.pathname.startsWith(`${to}/`);

  return (
    <aside className="sidebar">
      <nav className="sidebar-nav">
        {NAV_ITEMS.map((item) => {
          const isActive = isPathActive(item.to);

          return (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive: navIsActive }) =>
                `sidebar-link ${navIsActive ? "active" : ""}`
              }
              onPointerDownCapture={(e) => {
                if (!isActive) return;
                e.preventDefault();
              }}
              onMouseDown={(e) => {
                if (!isActive) return;
                e.preventDefault();
              }}
              onClickCapture={(e) => {
                if (!isActive) return;
                e.preventDefault();
                window.dispatchEvent(
                  new CustomEvent("app:active-nav-click", {
                    detail: { path: item.to },
                  })
                );
              }}
            >
              {item.label}
            </NavLink>
          );
        })}
      </nav>
    </aside>
  );
}

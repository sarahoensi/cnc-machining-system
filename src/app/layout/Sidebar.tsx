import { NavLink, useLocation } from "react-router-dom";
import "./Sidebar.css";

const NAV_ITEMS = [
  { to: "/triangle", label: "Triangle" },
  { to: "/helix", label: "Helix" },
  { to: "/cutting", label: "Cutting Data" },
  { to: "/cylinder-weight", label: "Cylinder Weight" },
  { to: "/finishing", label: "Finishing" },
];

export function Sidebar() {
  const location = useLocation();

  return (
    <aside className="sidebar">
      <nav className="sidebar-nav">
        {NAV_ITEMS.map((item) => {
          const isActive = location.pathname === item.to;

          return (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive: navIsActive }) =>
                `sidebar-link ${navIsActive ? "active" : ""}`
              }
              onMouseDown={(e) => {
                if (!isActive) return;
                e.preventDefault();
              }}
              onClick={(e) => {
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

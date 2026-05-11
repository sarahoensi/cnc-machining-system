// src/app/layout/Sidebar.tsx

import { NavLink } from "react-router-dom";
import "./Sidebar.css";

const NAV_ITEMS = [
  { to: "/triangle", label: "Triangle" },
  { to: "/helix", label: "Helix" },
  { to: "/cutting", label: "Cutting Data" },
  { to: "/cylinder-weight", label: "Cylinder Weight" },
  { to: "/finishing", label: "Finishing" },
];

export function Sidebar() {
  return (
    <aside className="sidebar">
      <nav className="sidebar-nav">
        {NAV_ITEMS.map(item => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              `sidebar-link ${isActive ? "active" : ""}`
            }
          >
            {item.label}
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}

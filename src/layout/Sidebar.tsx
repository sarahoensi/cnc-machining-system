import { NavLink } from "react-router-dom";

export function Sidebar() {
  return (
    <aside className="sidebar">
      <NavLink to="/right-triangle">Triangle</NavLink>
      <NavLink to="/helix">Helix</NavLink>
      <NavLink to="/cutting-data">Cutting</NavLink>
      <NavLink to="/finishing">Finishing</NavLink>
    </aside>
  );
}

import { useTheme } from "../app/providers/ThemeProvider";
//import "./Topbar.css";

type Props = {
  title?: string;
  rightSlot?: React.ReactNode;
};

export function Topbar({ title = "CNC System", rightSlot }: Props) {
  const { theme, setTheme } = useTheme();

  return (
    <header className="topbar">
      <div className="topbar-left">
        <h1 className="topbar-title">{title}</h1>
      </div>

      <div className="topbar-right">
        {rightSlot}

        <div className="theme-switch">
          <select
            value={theme}
            onChange={(e) => setTheme(e.target.value as any)}
            className="theme-select"
          >
            <option value="default">Default</option>
            <option value="dark">Dark</option>
            <option value="forest">Forest</option>
            <option value="pink">Pink</option>
          </select>
        </div>
      </div>
    </header>
  );
}

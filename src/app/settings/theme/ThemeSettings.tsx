import { useTheme } from "@app/providers/ThemeProvider";

export function ThemeSettings() {
  const { theme, setTheme } = useTheme();

  return (
    <>
      <button
        className={theme === "default" ? "active" : ""}
        onClick={() => setTheme("default")}
      >
        🌤 Standard
      </button>

      <button
        className={theme === "pink" ? "active" : ""}
        onClick={() => setTheme("pink")}
      >
        🌸 Rosa
      </button>

      <button
        className={theme === "forest" ? "active" : ""}
        onClick={() => setTheme("forest")}
      >
        🌲 Forest
      </button>

      <button
        className={theme === "dark" ? "active" : ""}
        onClick={() => setTheme("dark")}
      >
        🌙 Dark
      </button>
    </>
  );
}

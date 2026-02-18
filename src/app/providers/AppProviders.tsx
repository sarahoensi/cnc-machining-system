import { ThemeProvider } from "./ThemeProvider";
import { DisplaySettingProvider } from "./DisplaySettingProvider";

type Props = {
  children: React.ReactNode;
};

export function AppProviders({ children }: Props) {
  return (
    <ThemeProvider>
      <DisplaySettingProvider>
        {children}
      </DisplaySettingProvider>
    </ThemeProvider>
  );
}

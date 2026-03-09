import { ThemeProvider } from "./ThemeProvider";
import { DisplaySettingProvider } from "./DisplaySettingProvider";
import { FormStateProvider } from "./FormStateProvider";

type Props = {
  children: React.ReactNode;
};

export function AppProviders({ children }: Props) {
  return (
    <ThemeProvider>
      <DisplaySettingProvider>
        <FormStateProvider>
          {children}
        </FormStateProvider>
      </DisplaySettingProvider>
    </ThemeProvider>
  );
}

import { ThemeProvider } from "./ThemeProvider";
import { DisplaySettingProvider } from "./DisplaySettingProvider";
import { FormStateProvider } from "./FormStateProvider";
import { TitleProvider } from "../shell/TitleContext";

type Props = {
  children: React.ReactNode;
};

export function AppProviders({ children }: Props) {
  return (
    <ThemeProvider>
      <DisplaySettingProvider>
        <TitleProvider>
          <FormStateProvider>
            {children}
          </FormStateProvider>
        </TitleProvider>
      </DisplaySettingProvider>
    </ThemeProvider>
  );
}

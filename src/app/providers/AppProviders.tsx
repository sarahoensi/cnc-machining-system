// src/app/providers/AppProviders.tsx

import { ThemeProvider } from "./ThemeProvider";
import { DisplaySettingProvider } from "./DisplaySettingProvider";
import { FormStateProvider } from "./FormStateProvider";
import { TitleContextProvider } from "./TitleContextProvider";

type Props = {
  children: React.ReactNode;
};

export function AppProviders({ children }: Props) {
  return (
    <ThemeProvider>
      <DisplaySettingProvider>
        <TitleContextProvider>
          <FormStateProvider>{children}</FormStateProvider>
        </TitleContextProvider>
      </DisplaySettingProvider>
    </ThemeProvider>
  );
}

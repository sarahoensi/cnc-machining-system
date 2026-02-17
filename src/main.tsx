import React from "react";
import ReactDOM from "react-dom/client";
import App from "@app/App";
import { BrowserRouter } from "react-router-dom";
import { AppProviders } from "@app/providers/AppProviders";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <AppProviders>
        <App />
      </AppProviders>
    </BrowserRouter>
  </React.StrictMode>
);

import { useState } from "react";
import { NumberField } from "@shared/ui/components/form/NumberField/NumberField";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import { FieldState } from "@shared/types/fields";
import {
  CalculateButton,
  ResetButton,
  SettingsButton,
} from "@shared/ui/components/primitives/Button/Button";

type Props = {
  title: string;
};

export function PlaceholderPage({ title }: Props) {
  /* =================================================
     LIVE USER FIELD (for interaction test)
  ================================================= */

  const [userValue, setUserValue] = useState("123");

  const userField: FieldState = {
    value: userValue,
    source: userValue ? "user" : "empty",
    locked: false,
    invalid: false,
  };

  /* =================================================
     MODE SELECTOR STATE
  ================================================= */

  const [mode, setMode] = useState<"a" | "b">("a");

  return (
    <div
      style={{
        padding: "var(--space-0) var(--space-5)",
        display: "flex",
        flexDirection: "column",
        maxWidth: 700,
      }}
    >
      <h1
        style={{
          fontSize: "var(--font-size-xl)",
          fontWeight: "var(--font-weight-bold)",
        }}
      >
        {title}
      </h1>

      {/* ================================================= */}
      {/* NUMBER FIELD – SOURCE STATES                     */}
      {/* ================================================= */}

      <section
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-6)",
        }}
      >
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          NumberField – Source States
        </h2>

        {/* USER (live editable) */}
        <NumberField
          label="User source (editable)"
          field={userField}
          onChange={setUserValue}
          unit="mm"
          tooltip="Dette er et normalt brukerfelt"
        />

        {/* MACHINE */}
        <NumberField
          label="Machine source (readonly)"
          field={{
            value: "456",
            source: "machine",
            locked: false,
            invalid: false,
          }}
          onChange={() => {}}
          unit="mm"
          readonly
        />

        {/* LOCKED */}
        <NumberField
          label="Locked"
          field={{
            value: "789",
            source: "user",
            locked: true,
            invalid: false,
          }}
          onChange={() => {}}
          unit="mm"
          disabled
        />

        {/* EMPTY */}
        <NumberField
          label="Empty source"
          field={{
            value: "",
            source: "empty",
            locked: false,
            invalid: false,
          }}
          onChange={() => {}}
          unit="mm"
        />

        {/* ERROR */}
        <NumberField
          label="Error state"
          field={{
            value: "999",
            source: "user",
            locked: false,
            invalid: true,
          }}
          onChange={() => {}}
          unit="mm"
          error="Ugyldig verdi"
        />
      </section>

      {/* ================================================= */}
      {/* MODE SELECTOR                                    */}
      {/* ================================================= */}

      <section
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-6)",
        }}
      >
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          ModeSelector
        </h2>

        <ModeSelector
          name="mode"
          label="Normal"
          value={mode}
          onChange={setMode}
          options={[
            { value: "a", label: "Modus A", tooltip: "Standardmodus" },
            { value: "b", label: "Modus B", tooltip: "Avansert modus" },
          ]}
        />

        <ModeSelector
          name="mode-readonly"
          label="Readonly"
          value={mode}
          onChange={setMode}
          readonly
          options={[
            { value: "a", label: "Modus A" },
            { value: "b", label: "Modus B" },
          ]}
        />

        <ModeSelector
          name="mode-disabled"
          label="Disabled"
          value={mode}
          onChange={setMode}
          disabled
          options={[
            { value: "a", label: "Modus A" },
            { value: "b", label: "Modus B" },
          ]}
        />
      </section>

      {/* ================================================= */}
      {/* BUTTONS                                          */}
      {/* ================================================= */}

      <section
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-6)",
        }}
      >
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          Buttons
        </h2>

        <div
          style={{
            display: "flex",
            gap: "var(--space-3)",
            flexWrap: "wrap",
          }}
        >
          <CalculateButton />
          <ResetButton />
          <SettingsButton />
        </div>
      </section>
    </div>
  );
}

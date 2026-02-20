import { useState } from "react";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import type { FieldState } from "@shared/types/fields";
import {
  CalculateButton,
  ResetButton,
  SettingsButton,
} from "@shared/ui/components/primitives/Button/Button";
import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";
import { NumberInput } from "@shared/ui/components/primitives/NumberInput/NumberInput";

import {
  Table,
  TableHeader,
  TableHeaderSelect,
} from "@shared/ui/components/data/Table";

type Props = {
  title: string;
};

export function PlaceholderPage({ title }: Props) {
  /* ==============================
     STATE
  ============================== */

  const [userValue, setUserValue] = useState("123");

  const userField: FieldState = {
    value: userValue,
    source: userValue ? "user" : "empty",
    locked: false,
    invalid: false,
  };

  const [mode, setMode] = useState<"a" | "b">("a");

  const [deltaMode, setDeltaMode] =
    useState<"dd" | "dz">("dd");

  const [values, setValues] =
    useState<Record<number, FieldState>>({
      1: {
        value: "2.444",
        source: "user",
        locked: false,
        invalid: false,
      },
      2: {
        value: "",
        source: "empty",
        locked: false,
        invalid: false,
      },
      3: {
        value: "",
        source: "empty",
        locked: false,
        invalid: false,
      },
    });

  return (
    <div className="app-content split">
      {/* ================================================= */}
      {/* LEFT PANEL                                       */}
      {/* ================================================= */}

      <div className="app-left">
        <h1
          style={{
            fontSize: "var(--font-size-xl)",
            fontWeight: "var(--font-weight-bold)",
            marginBottom: "var(--space-6)",
          }}
        >
          {title}
        </h1>

        {/* Number fields */}
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

          <FormNumberField
            label="User source"
            tooltip="Dette kan redigeres manuelt"
            field={userField}
            onChange={setUserValue}
            unit="mm"
          />

          <FormNumberField
            label="Machine source"
            field={{
              value: "456",
              source: "machine",
              locked: false,
              invalid: false,
            }}
            onChange={() => {}}
            unit="mm"
          />
        </section>

        {/* Mode selector */}
        <section
          style={{
            marginTop: "var(--space-8)",
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
              { value: "a", label: "Modus A" },
              { value: "b", label: "Modus B" },
            ]}
          />
        </section>

        {/* Buttons */}
        <section
          style={{
            marginTop: "var(--space-8)",
            display: "flex",
            gap: "var(--space-3)",
            flexWrap: "wrap",
          }}
        >
          <CalculateButton />
          <ResetButton />
          <SettingsButton />
        </section>
      </div>

      {/* ================================================= */}
      {/* RIGHT PANEL                                      */}
      {/* ================================================= */}

      <div className="app-right">
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          Table Preview
        </h2>

        <Table.Root>
          <Table.Head>
            <Table.Row>
              <TableHeader label="Steg" />
              <TableHeader label="Start Ø" align="right" />

              <TableHeaderSelect
                value={deltaMode}
                onChange={setDeltaMode}
                options={[
                  { value: "dd", label: "ΔD" },
                  { value: "dz", label: "ΔZ" },
                ]}
                align="center"
              />

              <TableHeader label="Ny måling" align="right" />
              <Table.HeaderCell />
            </Table.Row>
          </Table.Head>

          <Table.Body>
            {[1, 2, 3].map((step) => (
              <Table.Row key={step}>
                <Table.Cell>{step}</Table.Cell>

                <Table.Cell align="right">
                  {step === 1 ? "2.000" : "—"}
                </Table.Cell>

                <Table.Cell align="center">
                  {deltaMode === "dd"
                    ? "0.444"
                    : "0.222"}
                </Table.Cell>

                <Table.Cell align="right">
                  <NumberInput
                    field={values[step]}
                    onChange={(val) =>
                      setValues((prev) => ({
                        ...prev,
                        [step]: {
                          ...prev[step],
                          value: val,
                        },
                      }))
                    }
                    unit="mm"
                    readonly={false}
                  />
                </Table.Cell>

                <Table.Cell align="center">
                  <button>Handling</button>
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      </div>
    </div>
  );
}
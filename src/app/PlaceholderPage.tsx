import { useState } from "react";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import { FieldState } from "@shared/types/fields";
import {
  CalculateButton,
  ResetButton,
  SettingsButton,
} from "@shared/ui/components/primitives/Button/Button";
import { ExecutionTable } from "@shared/ui/components/data/ExecutionTable/ExecutionTable";
import { ExecutionRow } from "@shared/ui/components/data/ExecutionTable/ExecutionRow";
import { TableHeaderCell } from "@shared/ui/components/data/TableHeader/TableHeaderCell";
import { FormNumberField } from "@shared/ui/components/form/FormNumberField/FormNumberField";
import { NumberInput } from "@shared/ui/components/primitives/NumberInput/NumberInput";

type Props = {
  title: string;
};

export function PlaceholderPage({ title }: Props) {
  /* =================================================
     USER FIELD
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

  /* =================================================
     EXECUTION TABLE STATE
  ================================================= */

  const [deltaMode, setDeltaMode] = useState<"dd" | "dz">("dz");

  return (
    <div
      style={{
        padding: "var(--space-0) var(--space-5)",
        display: "flex",
        flexDirection: "column",
        maxWidth: 900,
      }}
    >
      <h1
        style={{
          fontSize: "var(--font-size-xl)",
          fontWeight: "var(--font-weight-bold)",
          marginBottom: "var(--space-6)",
        }}
      >
        {title}
      </h1>

      {/* ================================================= */}
      {/* NUMBER FIELD                                     */}
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

        <FormNumberField
          label="User source (editable)"
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
          onChange={() => { }}
          unit="mm"

        />

        <FormNumberField
          label="Locked"
          field={{
            value: "789",
            source: "user",
            locked: true,
            invalid: false,
          }}
          onChange={() => { }}
          unit="mm"
          disabled
        />

        <FormNumberField
          label="Empty source"
          field={{
            value: "",
            source: "empty",
            locked: false,
            invalid: false,
          }}
          onChange={() => { }}
          unit="mm"
        />

        <FormNumberField
          label="Error state"
          field={{
            value: "999",
            source: "user",
            locked: false,
            invalid: true,
          }}
          onChange={() => { }}
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
          marginTop: "var(--space-8)",
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
      {/* EXECUTION TABLE                                  */}
      {/* ================================================= */}

      <section
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-6)",
          marginTop: "var(--space-8)",
        }}
      >
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          ExecutionTable
        </h2>

        <ExecutionTable
          headers={[
            <TableHeaderCell key="step" label="Steg" />,
            <TableHeaderCell key="start" label="Start Ø" align="right" />,

            <TableHeaderCell key="delta">
              <div className="header-select-wrapper">
                <select
                  className="header-select"
                  value={deltaMode}
                  onChange={(e) =>
                    setDeltaMode(e.target.value as "dd" | "dz")
                  }
                >
                  <option value="dd">ΔD</option>
                  <option value="dz">ΔZ</option>
                </select>
                <span className="header-select-caret" />
              </div>
            </TableHeaderCell>
            ,

            <TableHeaderCell key="new" label="Ny måling" align="center" />,
          ]}
        >
          <ExecutionRow>
            <td>1</td>
            <td style={{ textAlign: "right" }}>2.000</td>
            <td style={{ textAlign: "right" }}>0.444</td>
            <td>
              <NumberInput
                field={{
                  value: "2.444",
                  source: "user",
                  locked: false,
                  invalid: false,
                }}
                onChange={() => { }}
              />
            </td>
          </ExecutionRow>

          <ExecutionRow>
            <td>2</td>
            <td style={{ textAlign: "right" }}>—</td>
            <td style={{ textAlign: "right" }}>—</td>
            <td>
              <NumberInput
                field={{
                  value: "",
                  source: "empty",
                  locked: false,
                  invalid: false,
                }}
                onChange={() => { }}
              />
            </td>
          </ExecutionRow>
        </ExecutionTable>
      </section>

      {/* ================================================= */}
      {/* BUTTONS                                          */}
      {/* ================================================= */}

      <section
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-6)",
          marginTop: "var(--space-8)",
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
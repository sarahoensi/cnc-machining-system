import { useState } from "react";
import { ModeSelector } from "@shared/ui/components/form/ModeSelector/ModeSelector";
import type { FieldState } from "@shared/types/fields";
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

import { Table, TableHeaderSelect } from "@shared/ui/components/data/Table";
import { TableHeader } from "@shared/ui/components/data/Table";

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
     MODE SELECTOR (BEHOLDT)
  ================================================= */

  const [mode, setMode] = useState<"a" | "b">("a");

  /* =================================================
     EXECUTION TABLE STATE
  ================================================= */

  const [deltaMode, setDeltaMode] =
    useState<"dd" | "dz">("dz");

  /* =================================================
     TABLE DESIGN SYSTEM DEMO STATE
  ================================================= */

  const [tableMode, setTableMode] =
    useState<"deltaD" | "ae">("deltaD");

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
      {/* NUMBER FIELDS                                    */}
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
      {/* MODE SELECTOR (BEHOLDT)                          */}
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
      {/* EXECUTION TABLE (GAMMEL)                         */}
      {/* ================================================= */}

      <section style={{ marginTop: "var(--space-8)" }}>
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
            </TableHeaderCell>,
            <TableHeaderCell key="new" label="Ny måling" align="center" />,
          ]}
        >
          <ExecutionRow>
            <td>1</td>
            <td style={{ textAlign: "right" }}>2.000</td>
            <td style={{ textAlign: "right" }}>
              {deltaMode === "dd" ? "0.444" : "0.222"}
            </td>
            <td>
              <NumberInput
                field={values[1]}
                onChange={(val) =>
                  setValues((prev) => ({
                    ...prev,
                    1: { ...prev[1], value: val },
                  }))
                }
              />
            </td>
          </ExecutionRow>
        </ExecutionTable>
      </section>

      {/* ================================================= */}
      {/* NEW TABLE DESIGN SYSTEM                          */}
      {/* ================================================= */}

      <section style={{ marginTop: "var(--space-8)" }}>
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          Table Styling Preview
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
                  {deltaMode === "dd" ? "0.444" : "0.222"}
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
      </section>

      {/* ================================================= */}
      {/* BUTTONS                                          */}
      {/* ================================================= */}

      <section style={{ marginTop: "var(--space-8)" }}>
        <h2 style={{ fontSize: "var(--font-size-lg)" }}>
          Buttons
        </h2>

        <div style={{ display: "flex", gap: "var(--space-3)" }}>
          <CalculateButton />
          <ResetButton />
          <SettingsButton />
        </div>
      </section>
    </div>
  );
}
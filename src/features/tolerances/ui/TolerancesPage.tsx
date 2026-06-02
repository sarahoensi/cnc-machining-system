import { useState } from "react";

import { usePageTitle } from "@app/providers/TitleContextProvider";
import { getTauriCommandError } from "@shared/api/tauriError";
import { emptyField, userField } from "@shared/form/types/fields";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/components/form/fields/FormSelectMenuField";
import { Modal, ModalScrollArea } from "@shared/ui/components/overlay/Modal/Modal";
import { Table } from "@shared/ui/components/table/Table";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { Button } from "@shared/ui/primitives/Button/Button";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

import { calculateIso286FitApi } from "../api/client";
import type { Iso286FitResult } from "../api/types";
import { buildCalculateIso286FitRequest } from "../domain/buildRequest";
import { validateToleranceForm } from "../domain/validateToleranceForm";

import "./TolerancesPage.css";

const holeLetters = ["H", "JS"] as const;
const shaftLetters = ["g", "h", "js"] as const;
const toleranceGrades = ["6", "7"] as const;
const featureOptions = [
  { value: "hole", label: "Hole" },
  { value: "shaft", label: "Shaft" },
] as const;

type ActiveFeature = "hole" | "shaft";

export function TolerancesPage() {
  usePageTitle("Tolerances");

  const { decimals } = useDisplaySettings();
  const [nominal, setNominal] = useState("");
  const [activeFeature, setActiveFeature] = useState<ActiveFeature>("hole");
  const [holeLetter, setHoleLetter] = useState("H");
  const [holeGrade, setHoleGrade] = useState("7");
  const [shaftLetter, setShaftLetter] = useState("g");
  const [shaftGrade, setShaftGrade] = useState("6");
  const [fieldErrors, setFieldErrors] = useState<Record<string, string>>({});
  const [formError, setFormError] = useState<string | undefined>();
  const [result, setResult] = useState<Iso286FitResult | null>(null);
  const [tableOpen, setTableOpen] = useState(false);

  const activeLetter = activeFeature === "hole" ? holeLetter : shaftLetter;
  const activeGrade = activeFeature === "hole" ? holeGrade : shaftGrade;
  const activeLetterOptions = activeFeature === "hole" ? holeLetters : shaftLetters;
  const activeResult = activeFeature === "hole" ? result?.hole : result?.shaft;

  async function onCalculate() {
    const input = { nominal, holeLetter, holeGrade, shaftLetter, shaftGrade };
    const errors = validateToleranceForm(input);
    setFieldErrors(errors);
    setFormError(undefined);

    if (Object.keys(errors).length > 0) return;

    try {
      const response = await calculateIso286FitApi(
        buildCalculateIso286FitRequest(input),
      );
      setResult(response);
    } catch (error) {
      setFormError(getToleranceErrorMessage(error));
    }
  }

  function onReset() {
    setNominal("");
    setActiveFeature("hole");
    setHoleLetter("H");
    setHoleGrade("7");
    setShaftLetter("g");
    setShaftGrade("6");
    setFieldErrors({});
    setFormError(undefined);
    setResult(null);
    setTableOpen(false);
  }

  return (
    <>
      <div className="tolerances-page">
        <div className="tolerances-input-column">
          <FormSection>
            <FormModeField
              label="Feature"
              value={activeFeature}
              options={featureOptions}
              onChange={setActiveFeature}
            />

            <FormNumberField
              label="Nominal size"
              unit="mm"
              field={{
                ...userField(nominal),
                error: fieldErrors.nominal,
              }}
              onChange={setNominal}
            />

            <FormSelectMenuField
              label="Tolerance letter"
              valueLabel={activeLetter}
              options={activeLetterOptions.map((value) => ({
                value,
                label: value,
              }))}
              onSelect={(value) => {
                if (activeFeature === "hole") {
                  setHoleLetter(value);
                } else {
                  setShaftLetter(value);
                }
              }}
            />

            <FormSelectMenuField
              label="Tolerance grade"
              valueLabel={activeGrade}
              options={toleranceGrades.map((value) => ({
                value,
                label: value,
              }))}
              onSelect={(value) => {
                if (activeFeature === "hole") {
                  setHoleGrade(value);
                } else {
                  setShaftGrade(value);
                }
              }}
            />
          </FormSection>

          {formError && (
            <div className="tolerances-error">
              <FormError error={formError} />
            </div>
          )}

          <FormActions onCalculate={onCalculate} onReset={onReset} />
        </div>

        <div className="tolerances-output-column">
          <FormSection variant="result">
            <FormNumberField
              label="Minimum"
              unit="mm"
              field={machineField(activeResult?.min_mm)}
              readonly
              onChange={() => undefined}
            />
            <FormNumberField
              label="Maximum"
              unit="mm"
              field={machineField(activeResult?.max_mm)}
              readonly
              onChange={() => undefined}
            />
            <FormNumberField
              label="Min clearance"
              unit="mm"
              field={machineField(result?.fit.min_clearance_mm)}
              readonly
              onChange={() => undefined}
            />
            <FormNumberField
              label="Max clearance"
              unit="mm"
              field={machineField(result?.fit.max_clearance_mm)}
              readonly
              onChange={() => undefined}
            />
          </FormSection>

          <div className="tolerances-fit-type">
            {result ? `Fit type: ${result.fit.type}` : "Fit type: -"}
          </div>

          <div className="tolerances-result-actions">
            <Button
              variant="secondary"
              size="small"
              onClick={() => setTableOpen(true)}
              disabled={!result}
            >
              View in table
            </Button>
          </div>
        </div>
      </div>

      {tableOpen && result && (
        <Modal
          title="ISO 286 result table"
          onClose={() => setTableOpen(false)}
          size="lg"
          height="auto"
        >
          <ModalScrollArea>
            <ResultTable result={result} decimals={decimals} />
          </ModalScrollArea>
        </Modal>
      )}
    </>
  );
}

function getToleranceErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);
  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;
  return "ISO 286 fit calculation failed";
}

function machineField(value: number | undefined) {
  if (typeof value !== "number") return emptyField();

  return {
    ...emptyField(),
    source: "machine" as const,
    value: String(value),
    machineValue: value,
  };
}

function ResultTable({
  result,
  decimals,
}: {
  result: Iso286FitResult;
  decimals: number;
}) {
  return (
    <Table.Root className="tolerances-table">
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>Feature</Table.HeaderCell>
          <Table.HeaderCell>Code</Table.HeaderCell>
          <Table.HeaderCell align="right">Upper</Table.HeaderCell>
          <Table.HeaderCell align="right">Lower</Table.HeaderCell>
          <Table.HeaderCell align="right">Minimum</Table.HeaderCell>
          <Table.HeaderCell align="right">Maximum</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>
      <Table.Body>
        <Table.BodyRow>
          <Table.Cell>Hole</Table.Cell>
          <Table.Cell>{result.hole.code}</Table.Cell>
          <Table.Cell align="right">{result.hole.upper_um} um</Table.Cell>
          <Table.Cell align="right">{result.hole.lower_um} um</Table.Cell>
          <Table.Cell align="right">{formatNumber(result.hole.min_mm, decimals)} mm</Table.Cell>
          <Table.Cell align="right">{formatNumber(result.hole.max_mm, decimals)} mm</Table.Cell>
        </Table.BodyRow>
        <Table.BodyRow>
          <Table.Cell>Shaft</Table.Cell>
          <Table.Cell>{result.shaft.code}</Table.Cell>
          <Table.Cell align="right">{result.shaft.upper_um} um</Table.Cell>
          <Table.Cell align="right">{result.shaft.lower_um} um</Table.Cell>
          <Table.Cell align="right">{formatNumber(result.shaft.min_mm, decimals)} mm</Table.Cell>
          <Table.Cell align="right">{formatNumber(result.shaft.max_mm, decimals)} mm</Table.Cell>
        </Table.BodyRow>
      </Table.Body>
    </Table.Root>
  );
}

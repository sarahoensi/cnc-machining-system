/**
 * @vitest-environment jsdom
 */

import { DisplaySettingProvider } from "@app/providers/DisplaySettingProvider";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useRef, useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { userField } from "@shared/form/types/fields";

import {
  createInitialCylinderWeightForm,
  type CylinderWeightKey,
} from "../../domain/cylinderWeightForm";
import { CylinderWeightForm } from "./CylinderWeightForm";
import type { useCylinderWeightPageController } from "../useCylinderWeightPageController";
import type { CylinderMaterial } from "../materials";

type Controller = ReturnType<typeof useCylinderWeightPageController>;

const steel: CylinderMaterial = {
  id: "steel",
  name: "Steel",
  density_kg_m3: 7850,
};

const aluminum: CylinderMaterial = {
  id: "aluminum",
  name: "Aluminum",
  density_kg_m3: 2700,
};

function TestCylinderWeightForm({
  onCalculate = vi.fn(),
  onReset = vi.fn(),
  onMaterialChange = vi.fn(),
  onOpenManage = vi.fn(),
  onOpenCreate = vi.fn(),
}: {
  onCalculate?: () => void;
  onReset?: () => void;
  onMaterialChange?: (id: string) => void;
  onOpenManage?: () => void;
  onOpenCreate?: () => void;
}) {
  const [form, setForm] = useState(() => {
    const initial = createInitialCylinderWeightForm();
    return {
      ...initial,
      extras: {
        materialId: steel.id,
        materialName: steel.name,
        densityKgM3: steel.density_kg_m3,
      },
    };
  });
  const refs = useRef<Partial<Record<CylinderWeightKey, Element | null>>>({});
  const navigation = {
    containerRef: { current: null },
    register: (key: CylinderWeightKey) => (element: Element | null) => {
      refs.current[key] = element;
    },
    handleKeyDown: () => () => undefined,
    focusFirstAfterRender: vi.fn(),
    focusFirstInvalidAfterRender: vi.fn(),
    focusFirstInOrderAfterRender: vi.fn(),
  } as unknown as Controller["navigation"];

  const controller: Controller = {
    form,
    navigation,
    onFieldChange: (key, value) => {
      setForm((prev) => ({
        ...prev,
        fields: {
          ...prev.fields,
          [key]: {
            ...prev.fields[key],
            ...userField(value),
          },
        },
      }));
    },
    calculate: async () => {
      onCalculate();
      return form;
    },
    resetForm: onReset,
    materials: [steel, aluminum],
    selectedMaterial: steel,
    loadingMaterials: false,
    materialLoadError: undefined,
    onMaterialChange,
    manageModal: {
      open: false,
      setOpen: onOpenManage,
      newMaterialOpen: false,
      setNewMaterialOpen: onOpenCreate,
      exportOpen: false,
      setExportOpen: vi.fn(),
    },
    createMaterial: {
      name: "",
      setName: vi.fn(),
      density: "",
      setDensity: vi.fn(),
      error: undefined,
      setError: vi.fn(),
      save: vi.fn(),
    },
    editMaterial: {
      id: "",
      name: "",
      setName: vi.fn(),
      density: "",
      setDensity: vi.fn(),
      error: undefined,
      start: vi.fn(),
      cancel: vi.fn(),
      save: vi.fn(),
      remove: vi.fn(),
    },
    importExport: {
      importSummary: null,
      setImportSummary: vi.fn(),
      exportSummary: null,
      setExportSummary: vi.fn(),
      onImportMaterialsFile: vi.fn(),
      openExportDialog: vi.fn(),
      selectedExportIds: [],
      toggleExportMaterial: vi.fn(),
      setExportAll: vi.fn(),
      cancelExportDialog: vi.fn(),
      confirmExportSelected: vi.fn(),
    },
  };

  return (
    <DisplaySettingProvider>
      <CylinderWeightForm controller={controller} />
    </DisplaySettingProvider>
  );
}

describe("CylinderWeightForm", () => {
  it("lets the user enter dimensions and run form actions", async () => {
    const user = userEvent.setup();
    const onCalculate = vi.fn();
    const onReset = vi.fn();

    render(<TestCylinderWeightForm onCalculate={onCalculate} onReset={onReset} />);

    await user.type(screen.getByLabelText("Outer diameter"), "100");
    await user.type(screen.getByLabelText("Inner diameter"), "40");
    await user.type(screen.getByLabelText("Length"), "500");

    expect(screen.getByLabelText("Outer diameter")).toHaveValue("100");
    expect(screen.getByLabelText("Inner diameter")).toHaveValue("40");
    expect(screen.getByLabelText("Length")).toHaveValue("500");

    await user.click(screen.getByRole("button", { name: "Calculate" }));
    await user.click(screen.getByRole("button", { name: "Clear form" }));

    expect(onCalculate).toHaveBeenCalledTimes(1);
    expect(onReset).toHaveBeenCalledTimes(1);
  });

  it("lets the user select a material from the dropdown", async () => {
    const user = userEvent.setup();
    const onMaterialChange = vi.fn();

    render(<TestCylinderWeightForm onMaterialChange={onMaterialChange} />);

    await user.click(screen.getByRole("button", { name: /Steel/ }));
    await user.click(screen.getByRole("option", { name: /Aluminum/ }));

    expect(onMaterialChange).toHaveBeenCalledWith("aluminum");
  });

  it("opens material management actions from the form", async () => {
    const user = userEvent.setup();
    const onOpenManage = vi.fn();
    const onOpenCreate = vi.fn();

    render(
      <TestCylinderWeightForm
        onOpenManage={onOpenManage}
        onOpenCreate={onOpenCreate}
      />,
    );

    await user.click(screen.getByRole("button", { name: "Manage Materials" }));

    expect(onOpenManage).toHaveBeenCalledWith(true);

    await user.click(screen.getByRole("button", { name: /Steel/ }));
    await user.click(screen.getByRole("button", { name: "+ New Material..." }));

    expect(onOpenManage).toHaveBeenCalledWith(true);
    expect(onOpenCreate).toHaveBeenCalledWith(true);
  });
});

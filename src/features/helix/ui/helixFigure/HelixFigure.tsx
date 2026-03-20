import clsx from "clsx";
import "./HelixFigure.css";
import type { HelixKey } from "../../domain/helixForm";

/* --------------------------------------------- */
/* TYPES */
/* --------------------------------------------- */

type Props = {
    mode: "Inner" | "Outer";
    activeField?: HelixKey | null;
};

type VisualKey =
    | "helix"
    | "pitch"
    | "tool"
    | "holeDiameter";

type HelixSide = "front" | "back";

/* --------------------------------------------- */
/* CONFIG */
/* --------------------------------------------- */

const FIGURE = {
    viewBox: "0 0 240 220",

    center: {
        x: 120,
        topY: 30,
        height: 160,
    },

    helix: {
        turns: 4,
        ry: 14,
        rx: {
            Inner: 46,
            Outer: 60,
        },
    },

    centerSection: {
        x: 70,
        width: 100,
        offsetY: -20,
    },

    tool: {
        radius: 8,
        offsetTop: -10,
        offsetBottom: -6,
    },

    material: {
        leftX: 20,
        rightX: 220,
        baseHeight: 25,
    },
};

/* --------------------------------------------- */
/* FIELD MAPPING */
/* --------------------------------------------- */

const FIELD_TO_VISUAL: Partial<Record<HelixKey, VisualKey[]>> = {
    tool_diameter: ["tool"],
    pitch: ["pitch", "helix"],
    diameter: ["holeDiameter"],
};

function mapFieldToVisualKeys(
    field?: HelixKey | null
): VisualKey[] {
    if (!field) return [];
    return FIELD_TO_VISUAL[field] ?? [];
}

/* --------------------------------------------- */
/* HELIX PATH */
/* --------------------------------------------- */

function buildHelixPath({
    cx,
    topY,
    height,
    rx,
    ry,
    turns,
    side,
}: {
    cx: number;
    topY: number;
    height: number;
    rx: number;
    ry: number;
    turns: number;
    side: HelixSide;
}) {
    const steps = 240;
    let path = "";
    let lastSide: HelixSide | null = null;

    for (let i = 0; i <= steps; i++) {
        const u = i / steps;
        const t = u * turns * Math.PI * 2;

        const x = Math.cos(t) * rx;
        const y = Math.sin(t) * ry;
        const z = u * height;

        const px = cx + x;
        const py = topY + z + y * 0.5;

        const currentSide =
            Math.sin(t) > 0 ? "front" : "back";

        if (currentSide === side) {
            const cmd =
                path === "" || lastSide !== side
                    ? `M ${px} ${py}`
                    : `L ${px} ${py}`;

            path += cmd;
        }

        lastSide = currentSide;
    }

    return path;
}

/* --------------------------------------------- */
/* COMPONENT */
/* --------------------------------------------- */

export function HelixFigure({
    mode,
    activeField,
}: Props) {
    const activeKeys = mapFieldToVisualKeys(activeField);

    const isActive = (key: VisualKey) =>
        activeKeys.includes(key);

    const part = (key: VisualKey) =>
        clsx(
            "spiral-part",
            isActive(key) && "active"
        );

    /* ----------------------------------------- */
    /* DERIVED VALUES */
    /* ----------------------------------------- */

    const {
        center,
        helix,
        centerSection,
        tool,
        material,
    } = FIGURE;

    const rx = helix.rx[mode];
    const turnHeight = center.height / helix.turns;

    const sectionY = center.topY + centerSection.offsetY;

    const toolX =
        mode === "Inner"
            ? center.x + rx - 4
            : center.x + rx + tool.radius - 2;

    const toolTopY = center.topY + tool.offsetTop;
    const toolBottomY =
        center.topY + center.height + tool.offsetBottom;

    const centerLeft = centerSection.x;
    const centerRight =
        centerSection.x + centerSection.width;

    /* ----------------------------------------- */
    /* PATHS */
    /* ----------------------------------------- */

    const helixBack = buildHelixPath({
        cx: center.x,
        topY: center.topY,
        height: center.height,
        rx,
        ry: helix.ry,
        turns: helix.turns,
        side: "back",
    });

    const helixFront = buildHelixPath({
        cx: center.x,
        topY: center.topY,
        height: center.height,
        rx,
        ry: helix.ry,
        turns: helix.turns,
        side: "front",
    });

    /* ----------------------------------------- */
    /* RENDER */
    /* ----------------------------------------- */

    return (
        <svg
            viewBox={FIGURE.viewBox}
            className="spiral-figure"
            aria-hidden
        >
            {/* ---------------- MATERIAL ---------------- */}

            <rect
                x={material.leftX}
                y={center.topY + center.height}
                width={material.rightX - material.leftX}
                height={material.baseHeight}
                className="material"
            />

            {mode === "Inner" && (
                <>
                    <rect
                        x={material.leftX}
                        y={center.topY}
                        width={centerLeft - material.leftX}
                        height={center.height}
                        className="material"
                    />

                    <rect
                        x={centerRight}
                        y={center.topY}
                        width={
                            material.rightX - centerRight
                        }
                        height={center.height}
                        className="material"
                    />
                </>
            )}

            {mode === "Outer" && (
                <rect
                    x={centerLeft}
                    y={center.topY}
                    width={centerRight - centerLeft}
                    height={center.height}
                    className="material"
                />
            )}

            {/* ---------------- DIAMETER ---------------- */}

            <line
                x1={centerSection.x}
                y1={sectionY}
                x2={
                    centerSection.x +
                    centerSection.width
                }
                y2={sectionY}
                className={part("holeDiameter")}
            />

            <line
                x1={centerSection.x}
                y1={sectionY - 4}
                x2={centerSection.x}
                y2={sectionY + 4}
                className={part("holeDiameter")}
            />

            <line
                x1={
                    centerSection.x +
                    centerSection.width
                }
                y1={sectionY - 4}
                x2={
                    centerSection.x +
                    centerSection.width
                }
                y2={sectionY + 4}
                className={part("holeDiameter")}
            />

            {/* ---------------- HELIX BACK ---------------- */}

            <path d={helixBack} className="helix-back" />

            {/* ---------------- TOOL ---------------- */}

            <g
                className={clsx(
                    "tool",
                    isActive("tool") && "active"
                )}
            >
                <rect
                    x={toolX - tool.radius}
                    y={toolTopY}
                    width={tool.radius * 2}
                    height={toolBottomY - toolTopY}
                    className="tool-body-flat"
                />

                <line
                    x1={toolX - tool.radius}
                    y1={toolTopY}
                    x2={toolX + tool.radius}
                    y2={toolTopY}
                    className="tool-cap-line"
                />

                <path
                    d={`M ${toolX - tool.radius} ${toolBottomY}
                       L ${toolX} ${toolBottomY + 4}
                       L ${toolX + tool.radius} ${toolBottomY}`}
                    className="tool-tip"
                />
            </g>

            {/* ---------------- HELIX FRONT ---------------- */}

            <path
                d={helixFront}
                className={clsx(
                    "helix-front",
                    isActive("helix") && "active"
                )}
            />

            {/* ---------------- PITCH ---------------- */}

            <line
                x1="60"
                y1={center.topY + 20}
                x2="60"
                y2={center.topY + 20 + turnHeight}
                className={part("pitch")}
            />

            <line
                x1="56"
                y1={center.topY + 20}
                x2="64"
                y2={center.topY + 20}
                className={part("pitch")}
            />

            <line
                x1="56"
                y1={center.topY + 20 + turnHeight}
                x2="64"
                y2={center.topY + 20 + turnHeight}
                className={part("pitch")}
            />
        </svg>
    );
}
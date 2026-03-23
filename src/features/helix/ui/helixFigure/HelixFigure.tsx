import clsx from "clsx";
import "./HelixFigure.css";
import type { HelixKey } from "../../domain/helixForm";
import { buildToolPath } from "./parts/ToolPath";
import { buildHelixPath } from "./parts/HelixPath";
import { buildInnerMaterialPath } from "./parts/InnerMaterialPath";
import { buildOuterMaterialPath } from "./parts/OuterMaterialPath";
import { buildDiameterPath } from "./parts/DiameterPath";
import { buildPitchPath } from "./parts/PitchPath";

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


/* --------------------------------------------- */
/* MAP TO FIELDS
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
/* CONFIG */
/* --------------------------------------------- */

const FIGURE = {
    viewBox: `0 0 240 220`,

    centerBottom: {
        x: 240 / 2,
        y: 210,
    },

    centerSection: {
        radius: 50,
        height: 140,
    },

    materialBase: {
        baseWidth: 200,
        baseHeight: 25,
    },

    helix: {
        turns: 3,
        ry: 14,
    },

    tool: {
        toolRadius: 8,
        toolHeight: 160,
        offsetX: 10,
        offsetY: 6,
    },


};
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
            isActive(key) && "is-active"
        );

    /* ----------------------------------------- */
    /* DERIVED VALUES */
    /* ----------------------------------------- */

    const {
        centerBottom,
        helix,
        centerSection,
        tool,
        materialBase,
    } = FIGURE;

    const toolCenterX =
        mode === "Inner"
            ? centerBottom.x + centerSection.radius - tool.offsetX - tool.toolRadius
            : centerBottom.x + centerSection.radius + tool.offsetX + tool.toolRadius;

    const toolBottomY = centerBottom.y - materialBase.baseHeight - tool.offsetY;

    const rx = toolCenterX - centerBottom.x;

    const turnHeight = centerSection.height / helix.turns;

    const materialTopY = centerBottom.y - materialBase.baseHeight - centerSection.height;

    /* ----------------------------------------- */
    /* BUILD PATHS
    /* ----------------------------------------- */

    // TOOLPATH

    const toolPath = buildToolPath({
        toolCenterX,
        toolBottomY,
        toolRadius: tool.toolRadius,
        toolHeight: tool.toolHeight,
    })

    // MATERIAL

    const materialPath =
        mode === "Inner"
            ? buildInnerMaterialPath
            : buildOuterMaterialPath;

    const materialD = materialPath({
        centerBottom,

        baseWidth: materialBase.baseWidth,
        baseHeight: materialBase.baseHeight,

        materialTopY,
        centerRadius: centerSection.radius,
    });

    // HELIX
    const helixArgs = {
        cx: centerBottom.x,
        topY: materialTopY,
        height: centerSection.height,
        rx,
        ry: helix.ry,
        turns: helix.turns,
    };

    const helixBack = buildHelixPath({ ...helixArgs, side: "back" });
    const helixFront = buildHelixPath({ ...helixArgs, side: "front" });

    // DIAMETER

    const diameterPath = buildDiameterPath({
        radius: centerSection.radius,
        axisY: materialTopY,
        centerX: centerBottom.x,
        offsetY: 35,
    });


    // PITCH
    const pitchX = centerBottom.x - rx - 10;
    const turnIndex = 0.5;
    const y1 = materialTopY + turnIndex * turnHeight;
    const y2 = y1 + turnHeight;

    const pitchPath = buildPitchPath({
        x: pitchX,
        y1,
        y2,
    });



    return (
        <svg viewBox={FIGURE.viewBox} className="spiral-figure" aria-hidden>
            {/* MATERIAL */}
            <path d={materialD} className="material" />

            {/* DIAMETER */}
            <path d={diameterPath} className={part("holeDiameter")} />

            {/* HELIX BACK */}
            <path d={helixBack} className="helix-back" />

            {/* TOOL */}
            <path d={toolPath} className={clsx("tool", isActive("tool") && "is-active")} />

            {/* HELIX FRONT */}
            <path d={helixFront} className={"helix-front"} />
            
            {/* PITCH */}
            <path d={pitchPath} className={part("pitch")} />
        </svg>
    );
}
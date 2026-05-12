// src/features/helix/ui/helixFigure/parts/ToolPath.tsx

//
/* Tool */

export function buildToolPath({
    toolCenterX,
    toolBottomY,
    toolRadius,
    toolHeight
}: {
    toolCenterX: number;
    toolBottomY: number;
    toolRadius: number;
    toolHeight: number;
}) {
    
    const left = toolCenterX - toolRadius;
    const right = toolCenterX + toolRadius;

    const topY = toolBottomY - toolHeight;
    const bottomY = toolBottomY;

    return `
        M ${left} ${topY}
        L ${right} ${topY}
        L ${right} ${bottomY}
        L ${left} ${bottomY}
        Z
    `;
}

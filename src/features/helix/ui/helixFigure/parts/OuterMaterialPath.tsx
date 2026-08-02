// src/features/helix/ui/helixFigure/parts/OuterMaterialPath.tsx

//

export function buildOuterMaterialPath({
  centerBottom,

  baseWidth,
  baseHeight,

  materialTopY,
  centerRadius,
}: {
  centerBottom: { x: number; y: number };
  baseWidth: number;
  baseHeight: number;

  materialTopY: number;
  centerRadius: number;
}) {
  const baseLeft = centerBottom.x - baseWidth / 2;
  const baseRight = centerBottom.x + baseWidth / 2;

  const topLeftX = centerBottom.x - centerRadius;
  const topRightX = centerBottom.x + centerRadius;

  const topY = materialTopY;
  const baseTop = centerBottom.y - baseHeight;
  const baseBottom = centerBottom.y;

  return `
        M ${baseLeft} ${baseTop}
        L ${baseLeft} ${baseBottom}
        L ${baseRight} ${baseBottom}
        L ${baseRight} ${baseTop}

        L ${topRightX} ${baseTop}
        L ${topRightX} ${topY}
        L ${topLeftX} ${topY}
        L ${topLeftX} ${baseTop}

        Z
    `;
}

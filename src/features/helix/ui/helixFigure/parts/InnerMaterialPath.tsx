/* --------------------------------------------- */
/* InnerMaterial */
/* --------------------------------------------- */
export function buildInnerMaterialPath({
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
  const materialLeft = centerBottom.x - baseWidth / 2;
  const materialRight = centerBottom.x + baseWidth / 2;

  const baseBottom = centerBottom.y;
  const baseTop = centerBottom.y - baseHeight;

  const topY = materialTopY;

  const topLeftX = centerBottom.x - centerRadius;
  const topRightX = centerBottom.x + centerRadius;

  return `
        M ${materialLeft} ${topY}

        L ${materialLeft} ${baseBottom}
        L ${materialRight} ${baseBottom}
        L ${materialRight} ${topY}

        L ${topRightX} ${topY}
        L ${topRightX} ${baseTop}
        L ${topLeftX} ${baseTop}
        L ${topLeftX} ${topY}

        Z
    `;
}

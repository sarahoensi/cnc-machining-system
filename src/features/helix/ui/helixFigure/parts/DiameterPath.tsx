// features/helix/ui/helixfigure/parts/DiameterPath.tsx

export function buildDiameterPath({
  radius,
  axisY,
  centerX,
  offsetY,
}: {
  radius: number;
  axisY: number;
  centerX: number;
  offsetY: number;
}) {
  const left = centerX - radius;
  const right = centerX + radius;

  const y = axisY - offsetY;
  const topY = y - 4;
  const bottomY = y + 4;

  return `
        M ${left} ${y}
        L ${right} ${y}

        M ${left} ${topY}
        L ${left} ${bottomY}

        M ${right} ${topY}
        L ${right} ${bottomY}

        
    `;
}

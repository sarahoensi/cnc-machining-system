//

export function buildPitchPath({
    x,
    y1,
    y2,
}: {
    x: number;
    y1: number;
    y2: number;
}) {
    const tick = 4;

    return `
        M ${x} ${y1}
        L ${x} ${y2}

        M ${x - tick} ${y1}
        L ${x + tick} ${y1}

        M ${x - tick} ${y2}
        L ${x + tick} ${y2}
    `;
}
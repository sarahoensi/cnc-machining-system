//

/* --------------------------------------------- */
/* HELIX PATH */
/* --------------------------------------------- */

type HelixSide = "front" | "back";

export function buildHelixPath({
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
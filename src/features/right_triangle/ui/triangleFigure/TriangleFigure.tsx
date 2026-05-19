// features/right_triangle/ui/triangleFigure.tsx

import "./TriangleFigure.css";
import type { TriangleKey } from "../../domain/triangleForm";

type Props = {
  activeField: TriangleKey | null;
};

type Point = { x: number; y: number };

export function TriangleFigure({ activeField }: Props) {
  // Hjørner:
  // BL = bottom-left  (rett vinkel)
  // TL = top-left
  // BR = bottom-right
  const BL: Point = { x: 10, y: 100 };
  const TL: Point = { x: 10, y: 10 };
  const BR: Point = { x: 100, y: 100 };

  return (
    <div className="triangle-figure">
      <svg viewBox="0 0 120 120" width="100%">
        {/* Sider */}
        <Edge A={BL} B={BR} active={activeField === "a"} />
        <Edge A={TL} B={BL} active={activeField === "b"} />
        <Edge A={TL} B={BR} active={activeField === "c"} />

        {/* Rett vinkel mellom a og b, altså i BL */}
        <RightAngle vertex={BL} p1={BR} p2={TL} />

        {/* Labels */}
        <Label at={mid(BL, BR)} text="a" active={activeField === "a"} dy={12} />
        <Label at={mid(TL, BL)} text="b" active={activeField === "b"} dx={-8} />
        <Label at={mid(TL, BR)} text="c" active={activeField === "c"} dx={6} />

        {/* β mellom a og c, altså i BR */}
        <AngleArc
          vertex={BR}
          p1={BL}
          p2={TL}
          r={18}
          active={activeField === "beta"}
          label="β"
        />

        {/* α mellom b og c, altså i TL */}
        <AngleArc
          vertex={TL}
          p1={BL}
          p2={BR}
          r={18}
          active={activeField === "alpha"}
          label="α"
        />
      </svg>
    </div>
  );
}

function Edge({
  A,
  B,
  active,
}: {
  A: Point;
  B: Point;
  active: boolean;
}) {
  return (
    <line
      x1={A.x}
      y1={A.y}
      x2={B.x}
      y2={B.y}
      className={cls("edge", active)}
    />
  );
}

function Label({
  at,
  text,
  active,
  dx = 0,
  dy = 0,
}: {
  at: Point;
  text: string;
  active: boolean;
  dx?: number;
  dy?: number;
}) {
  return (
    <text x={at.x + dx} y={at.y + dy} className={cls("label", active)}>
      {text}
    </text>
  );
}

function AngleArc({
  vertex,
  p1,
  p2,
  r,
  active,
  label,
}: {
  vertex: Point;
  p1: Point;
  p2: Point;
  r: number;
  active: boolean;
  label?: string;
}) {
  const a1 = angle(vertex, p1);
  const a2 = angle(vertex, p2);

  const [start, end] = normalizeAngles(a1, a2);
  const d = describeArc(vertex.x, vertex.y, r, start, end);

  const midAngle = shortestMidAngle(start, end);
  const labelPos = {
    x: vertex.x + (r + 8) * Math.cos(midAngle),
    y: vertex.y - (r + 8) * Math.sin(midAngle),
  };

  return (
    <>
      <path d={d} className={cls("angle", active)} />
      {label && (
        <text
          x={labelPos.x}
          y={labelPos.y}
          className={cls("label", active)}
        >
          {label}
        </text>
      )}
    </>
  );
}

function RightAngle({
  vertex,
  p1,
  p2,
}: {
  vertex: Point;
  p1: Point;
  p2: Point;
}) {
  const size = 10;

  const dir1 = unit(vertex, p1);
  const dir2 = unit(vertex, p2);

  const q1 = add(vertex, scale(dir1, size));
  const q2 = add(q1, scale(dir2, size));
  const q3 = add(vertex, scale(dir2, size));

  return (
    <path
      d={`M ${q1.x} ${q1.y} L ${q2.x} ${q2.y} L ${q3.x} ${q3.y}`}
      className="right-angle"
    />
  );
}

// Bruk matematisk koordinatsystem i vinkelberegning:
// positiv y oppover, derfor inverterer vi SVG-y.
function angle(a: Point, b: Point) {
  return Math.atan2(-(b.y - a.y), b.x - a.x);
}

// Velg den minste vinkelen mellom to stråler.
function normalizeAngles(a1: number, a2: number): [number, number] {
  let diff = a2 - a1;

  while (diff <= -Math.PI) diff += Math.PI * 2;
  while (diff > Math.PI) diff -= Math.PI * 2;

  if (diff < 0) {
    return [a2, a1];
  }

  return [a1, a2];
}

function shortestMidAngle(start: number, end: number) {
  let diff = end - start;

  while (diff <= -Math.PI) diff += Math.PI * 2;
  while (diff > Math.PI) diff -= Math.PI * 2;

  return start + diff / 2;
}

function describeArc(
  cx: number,
  cy: number,
  r: number,
  start: number,
  end: number
) {
  const x1 = cx + r * Math.cos(start);
  const y1 = cy - r * Math.sin(start);

  const x2 = cx + r * Math.cos(end);
  const y2 = cy - r * Math.sin(end);

  let diff = end - start;
  while (diff <= -Math.PI) diff += Math.PI * 2;
  while (diff > Math.PI) diff -= Math.PI * 2;

  const largeArc = Math.abs(diff) > Math.PI ? 1 : 0;
  const sweep = diff >= 0 ? 0 : 1;

  return `M ${x1} ${y1} A ${r} ${r} 0 ${largeArc} ${sweep} ${x2} ${y2}`;
}

function mid(a: Point, b: Point): Point {
  return {
    x: (a.x + b.x) / 2,
    y: (a.y + b.y) / 2,
  };
}

function unit(a: Point, b: Point): Point {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const len = Math.hypot(dx, dy);
  return { x: dx / len, y: dy / len };
}

function scale(v: Point, s: number): Point {
  return { x: v.x * s, y: v.y * s };
}

function add(a: Point, b: Point): Point {
  return { x: a.x + b.x, y: a.y + b.y };
}

function cls(base: string, active: boolean) {
  return active ? `${base} is-active` : base;
}
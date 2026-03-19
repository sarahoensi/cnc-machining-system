// features/right_triangle/ui/triangleFigure.tsx

import "./TriangleFigure.css";
import type { TriangleKey } from "../../domain/triangleForm";

type Props = {
  activeField: TriangleKey | null;
};

export function TriangleFigure({ activeField }: Props) {
  return (
    <div className="triangle-figure">
      <svg viewBox="0 0 120 120">

        {/* Katet a */}
        <line
          x1="10" y1="100"
          x2="100" y2="100"
          className={cls("edge", activeField === "a")}
        />

        {/* Katet b */}
        <line
          x1="10" y1="10"
          x2="10" y2="100"
          className={cls("edge", activeField === "b")}
        />

        {/* Hypotenus c */}
        <line
          x1="10" y1="10"
          x2="100" y2="100"
          className={cls("edge", activeField === "c")}
        />

        {/* Rett vinkel markering */}
        <rect
          x="10"
          y="100"
          width="10"
          height="-10"
          className="right-angle"
        />

        {/* Labels */}

        <text
          x="55"
          y="112"
          className={cls("label", activeField === "a")}
        >
          a
        </text>

        <text
          x="2"
          y="55"
          className={cls("label", activeField === "b")}
        >
          b
        </text>

        <text
          x="60"
          y="55"
          className={cls("label", activeField === "c")}
        >
          c
        </text>

        {/* Alpha */}
        <path
          d="M 10 100 A 20 20 0 0 1 30 100"
          className={cls("angle", activeField === "alpha")}
        />

        <text
          x="25"
          y="95"
          className={cls("label", activeField === "alpha")}
        >
          α
        </text>

        {/* Beta */}
        <path
          d="M 10 10 A 20 20 0 0 0 10 30"
          className={cls("angle", activeField === "beta")}
        />

        <text
          x="15"
          y="30"
          className={cls("label", activeField === "beta")}
        >
          β
        </text>

      </svg>
    </div>
  );
}

/* helper */
function cls(base: string, active: boolean) {
  return active ? `${base} is-active` : base;
}
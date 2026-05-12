import clsx from "clsx";
import { ReactNode } from "react";
import "./FormSection.css";

type FormSectionProps = {
  children: ReactNode;
  variant?: "default" | "result";
  className?: string;
};

export function FormSection({
  children,
  variant = "default",
  className,
}: FormSectionProps) {
  return (
    <div
      className={clsx(
        "form-section",
        variant === "result" && "form-section--result",
        className
      )}
    >
      {children}
    </div>
  );
}

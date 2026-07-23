import type {
  ButtonHTMLAttributes,
  ReactNode,
} from "react";

import clsx from "clsx";

interface ButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  variant?: "primary" | "secondary" | "danger";
  loading?: boolean;
}

export function Button({
  children,
  variant = "primary",
  loading = false,
  className,
  disabled,
  ...props
}: ButtonProps) {
  return (
    <button
      {...props}
      disabled={disabled || loading}
      className={clsx(
        "button",
        `button--${variant}`,
        className,
      )}
    >
      {loading ? "Procesando..." : children}
    </button>
  );
}
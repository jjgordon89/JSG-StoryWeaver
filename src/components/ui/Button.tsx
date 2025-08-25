import React from "react";
import { cn } from "../../utils/cn";

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "primary" | "secondary" | "ghost" | "link" | "outline";
  size?: "default" | "sm" | "lg" | "icon";
}

const variantClasses: Record<NonNullable<ButtonProps["variant"]>, string> = {
  default:
    "bg-slate-900 text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-white",
  primary:
    "bg-pink-500 text-white hover:bg-pink-600 focus:ring-2 focus:ring-pink-300",
  secondary:
    "bg-slate-100 text-slate-900 hover:bg-slate-200 dark:bg-slate-800 dark:text-slate-100 dark:hover:bg-slate-700",
  ghost:
    "bg-transparent hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-900 dark:text-slate-100",
  link: "bg-transparent text-pink-600 hover:underline",
  outline:
    "bg-white/90 border border-slate-200/70 text-slate-900 hover:bg-white shadow-sm dark:bg-slate-800/80 dark:text-slate-100 dark:border-slate-700",
};

const sizeClasses: Record<NonNullable<ButtonProps["size"]>, string> = {
  default: "h-9 px-3 rounded-md",
  sm: "h-8 px-2.5 text-sm rounded-md",
  lg: "h-10 px-4 text-base rounded-lg",
  icon: "h-9 w-9 p-0 rounded-full justify-center",
};

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", size = "default", ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={cn(
          "inline-flex items-center gap-2 font-medium transition-colors focus:outline-none",
          variantClasses[variant],
          sizeClasses[size],
          className
        )}
        {...props}
      />
    );
  }
);
Button.displayName = "Button";

export default Button;

import { cn } from "@/lib/cn";

interface ButtonProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  variant?: "primary" | "secondary";
  children: React.ReactNode;
}

export function Button({
  variant = "primary",
  className,
  children,
  ...props
}: ButtonProps) {
  return (
    <a
      className={cn(
        "inline-flex items-center justify-center px-6 py-3 font-semibold rounded-lg transition-all duration-200 cursor-pointer",
        variant === "primary" &&
          "bg-terracotta text-white hover:bg-terracotta-hover hover:scale-[1.02] active:scale-[0.98]",
        variant === "secondary" &&
          "border border-navy/15 text-navy hover:bg-cream-dark hover:border-navy/25",
        className
      )}
      {...props}
    >
      {children}
    </a>
  );
}

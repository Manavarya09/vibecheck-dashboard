import { cn } from "@/lib/cn";

interface GradientTextProps {
  children: React.ReactNode;
  className?: string;
}

export function GradientText({ children, className }: GradientTextProps) {
  return (
    <span
      className={cn(
        "bg-gradient-to-r from-terracotta via-terracotta-light to-terracotta bg-clip-text text-transparent",
        className
      )}
    >
      {children}
    </span>
  );
}

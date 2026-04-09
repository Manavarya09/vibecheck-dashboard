import { cn } from "@/lib/cn";

interface AnimatedBorderProps {
  children: React.ReactNode;
  className?: string;
}

export function AnimatedBorder({ children, className }: AnimatedBorderProps) {
  return (
    <div className={cn("relative rounded-xl p-px overflow-hidden group", className)}>
      <div
        className="absolute inset-0 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"
        style={{
          background:
            "conic-gradient(from 0deg, var(--color-terracotta), var(--color-cream-dark), var(--color-terracotta))",
        }}
      />
      <div className="relative rounded-xl bg-cream">{children}</div>
    </div>
  );
}

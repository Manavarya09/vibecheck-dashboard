import { cn } from "@/lib/cn";

interface SeparatorProps {
  className?: string;
}

export function Separator({ className }: SeparatorProps) {
  return (
    <div
      className={cn("w-full max-w-5xl mx-auto px-6", className)}
    >
      <div className="h-px bg-gradient-to-r from-transparent via-border to-transparent" />
    </div>
  );
}

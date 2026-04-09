import { cn } from "@/lib/cn";

interface SectionHeadingProps {
  eyebrow?: string;
  title: string;
  description?: string;
  className?: string;
  align?: "left" | "center";
}

export function SectionHeading({
  eyebrow,
  title,
  description,
  className,
  align = "center",
}: SectionHeadingProps) {
  return (
    <div
      className={cn(
        "max-w-3xl",
        align === "center" && "mx-auto text-center",
        className
      )}
    >
      {eyebrow && (
        <p className="text-xs font-mono font-semibold uppercase tracking-[0.15em] text-terracotta mb-4">
          {eyebrow}
        </p>
      )}
      <h2 className="text-3xl md:text-5xl font-bold tracking-tight text-navy leading-[1.1]">
        {title}
      </h2>
      {description && (
        <p className="mt-4 text-lg text-navy-light leading-relaxed">
          {description}
        </p>
      )}
    </div>
  );
}

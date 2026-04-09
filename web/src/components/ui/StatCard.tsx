"use client";

import { useCountUp } from "@/hooks/useCountUp";
import { useInView } from "@/hooks/useInView";
import { Citation } from "@/components/ui/Citation";

interface StatCardProps {
  value: number;
  suffix: string;
  label: string;
  citation: string;
  decimals?: number;
}

export function StatCard({
  value,
  suffix,
  label,
  citation,
  decimals = 0,
}: StatCardProps) {
  const { ref, isInView } = useInView(0.3);
  const count = useCountUp(value, 2000, isInView, decimals);

  return (
    <div ref={ref} className="flex flex-col group">
      <div className="flex items-baseline gap-1">
        <span className="text-5xl md:text-6xl font-bold font-mono text-terracotta tabular-nums transition-transform duration-300 group-hover:-translate-y-1">
          {isInView ? count : 0}
        </span>
        <span className="text-2xl md:text-3xl font-bold text-terracotta">
          {suffix}
        </span>
      </div>
      <p className="mt-3 text-base text-navy-light leading-relaxed">{label}</p>
      <Citation text={citation} />
    </div>
  );
}

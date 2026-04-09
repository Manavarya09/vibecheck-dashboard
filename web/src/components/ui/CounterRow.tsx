"use client";

import { useCountUp } from "@/hooks/useCountUp";
import { useInView } from "@/hooks/useInView";

interface CounterRowProps {
  items: { value: number; suffix: string; label: string }[];
}

export function CounterRow({ items }: CounterRowProps) {
  const { ref, isInView } = useInView(0.3);

  return (
    <div ref={ref} className="flex flex-wrap justify-center gap-12 md:gap-16">
      {items.map((item, i) => (
        <CounterItem key={i} {...item} started={isInView} />
      ))}
    </div>
  );
}

function CounterItem({
  value,
  suffix,
  label,
  started,
}: {
  value: number;
  suffix: string;
  label: string;
  started: boolean;
}) {
  const count = useCountUp(value, 1800, started, suffix === "%" ? 1 : 0);

  return (
    <div className="text-center">
      <p className="text-3xl md:text-4xl font-bold font-mono text-navy">
        {started ? count : 0}
        <span className="text-terracotta">{suffix}</span>
      </p>
      <p className="mt-1 text-sm text-navy-muted">{label}</p>
    </div>
  );
}

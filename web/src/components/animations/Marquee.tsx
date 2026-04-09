"use client";

import { cn } from "@/lib/cn";

interface MarqueeProps {
  children: React.ReactNode;
  className?: string;
  speed?: number;
  direction?: "left" | "right";
}

export function Marquee({
  children,
  className,
  speed = 40,
  direction = "left",
}: MarqueeProps) {
  const animationDirection = direction === "left" ? "normal" : "reverse";

  return (
    <div className={cn("overflow-hidden", className)}>
      <div
        className="flex gap-8 whitespace-nowrap"
        style={{
          animation: `marquee ${speed}s linear infinite`,
          animationDirection,
        }}
      >
        {children}
        {children}
      </div>
      <style>{`
        @keyframes marquee {
          from { transform: translateX(0); }
          to { transform: translateX(-50%); }
        }
      `}</style>
    </div>
  );
}

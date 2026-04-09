"use client";

import { motion, useScroll, useTransform } from "framer-motion";
import { useRef } from "react";

interface LineDrawingProps {
  className?: string;
  direction?: "vertical" | "horizontal";
}

export function LineDrawing({
  className,
  direction = "vertical",
}: LineDrawingProps) {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start 0.8", "end 0.2"],
  });

  const scaleY = useTransform(scrollYProgress, [0, 1], [0, 1]);
  const scaleX = useTransform(scrollYProgress, [0, 1], [0, 1]);

  return (
    <div ref={ref} className={className}>
      <motion.div
        className={
          direction === "vertical"
            ? "w-px bg-terracotta/30 origin-top"
            : "h-px bg-terracotta/30 origin-left"
        }
        style={
          direction === "vertical"
            ? { scaleY, height: "100%" }
            : { scaleX, width: "100%" }
        }
      />
    </div>
  );
}

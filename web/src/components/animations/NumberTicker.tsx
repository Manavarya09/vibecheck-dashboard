"use client";

import { motion, useInView, useMotionValue, useTransform, animate } from "framer-motion";
import { useEffect, useRef } from "react";

interface NumberTickerProps {
  value: number;
  suffix?: string;
  prefix?: string;
  decimals?: number;
  className?: string;
  duration?: number;
}

export function NumberTicker({
  value,
  suffix = "",
  prefix = "",
  decimals = 0,
  className,
  duration = 2,
}: NumberTickerProps) {
  const ref = useRef<HTMLSpanElement>(null);
  const isInView = useInView(ref, { once: true, margin: "-100px" });
  const motionValue = useMotionValue(0);
  const display = useTransform(motionValue, (v) =>
    `${prefix}${v.toFixed(decimals)}${suffix}`
  );

  useEffect(() => {
    if (isInView) {
      animate(motionValue, value, {
        duration,
        ease: [0.25, 0.4, 0.25, 1],
      });
    }
  }, [isInView, value, duration, motionValue]);

  return (
    <motion.span ref={ref} className={className}>
      {display}
    </motion.span>
  );
}

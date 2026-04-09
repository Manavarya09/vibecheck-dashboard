"use client";

import { motion } from "framer-motion";

interface FadeInProps {
  children: React.ReactNode;
  className?: string;
  delay?: number;
  duration?: number;
  direction?: "up" | "down" | "none";
}

export function FadeIn({
  children,
  className,
  delay = 0,
  duration = 0.7,
  direction = "up",
}: FadeInProps) {
  const y = direction === "up" ? 20 : direction === "down" ? -20 : 0;

  return (
    <motion.div
      className={className}
      initial={{ opacity: 0, y }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: "-60px" }}
      transition={{ duration, delay, ease: [0.25, 0.4, 0.25, 1] as const }}
    >
      {children}
    </motion.div>
  );
}

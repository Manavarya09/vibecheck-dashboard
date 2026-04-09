"use client";

import { motion } from "framer-motion";
import { cn } from "@/lib/cn";

interface KineticHeadlineProps {
  text: string;
  className?: string;
  delay?: number;
  tag?: "h1" | "h2" | "h3";
}

const wordVariants = {
  hidden: {
    opacity: 0,
    y: 40,
    rotateX: -40,
  },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    rotateX: 0,
    transition: {
      duration: 0.7,
      delay: i * 0.08,
      ease: [0.25, 0.4, 0.25, 1],
    },
  }),
};

export function KineticHeadline({
  text,
  className,
  delay = 0,
  tag = "h1",
}: KineticHeadlineProps) {
  const words = text.split(" ");
  const Tag = tag;

  return (
    <Tag className={cn("overflow-hidden", className)}>
      <motion.span
        className="inline-flex flex-wrap justify-center gap-x-[0.3em]"
        initial="hidden"
        whileInView="visible"
        viewport={{ once: true }}
        style={{ perspective: 600 }}
      >
        {words.map((word, i) => (
          <motion.span
            key={i}
            custom={i + delay / 0.08}
            variants={wordVariants}
            className="inline-block origin-bottom"
            style={{ transformStyle: "preserve-3d" }}
          >
            {word}
          </motion.span>
        ))}
      </motion.span>
    </Tag>
  );
}

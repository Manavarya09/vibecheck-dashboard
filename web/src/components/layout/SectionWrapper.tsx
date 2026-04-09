"use client";

import { motion } from "framer-motion";
import { cn } from "@/lib/cn";

interface SectionWrapperProps {
  children: React.ReactNode;
  className?: string;
  id?: string;
  delay?: number;
}

const variants = {
  hidden: { opacity: 0, y: 30 },
  visible: (delay: number) => ({
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.8,
      delay,
      ease: [0.25, 0.4, 0.25, 1] as const,
    },
  }),
};

export function SectionWrapper({
  children,
  className,
  id,
  delay = 0,
}: SectionWrapperProps) {
  return (
    <motion.section
      id={id}
      className={cn("py-20 md:py-28 lg:py-32 px-6 md:px-8", className)}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-100px" }}
      custom={delay}
      variants={variants}
    >
      {children}
    </motion.section>
  );
}

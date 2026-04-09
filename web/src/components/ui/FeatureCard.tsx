"use client";

import { motion } from "framer-motion";

interface FeatureCardProps {
  title: string;
  description: string;
}

export function FeatureCard({ title, description }: FeatureCardProps) {
  return (
    <motion.div
      className="p-6 rounded-xl border border-border bg-cream transition-colors duration-200"
      whileHover={{ scale: 1.02, backgroundColor: "var(--color-cream-dark)" }}
      transition={{ duration: 0.2 }}>
      <h3 className="text-base font-semibold text-navy">{title}</h3>
      <p className="mt-2 text-sm text-navy-light leading-relaxed">
        {description}
      </p>
    </motion.div>
  );
}

"use client";

import { motion } from "framer-motion";
import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { FadeIn } from "@/components/animations/FadeIn";
import { AIFL_PHASES } from "@/lib/constants";

export function AiflFramework() {
  return (
    <SectionWrapper id="framework" className="bg-navy text-cream">
      <div className="max-w-5xl mx-auto">
        <SectionHeading
          eyebrow="The Framework"
          title="The AI-Induced Flow Loop"
          description="A six-phase self-reinforcing engagement cycle that explains why vibe coding is uniquely absorbing. VibeCheck detects and interrupts this loop at each phase."
          className="[&_p]:text-cream/60 [&_h2]:text-cream"
        />

        <div className="mt-16 space-y-0">
          {AIFL_PHASES.map((phase, i) => (
            <FadeIn key={i} delay={i * 0.1}>
              <div className="group flex gap-6 md:gap-10 py-6 border-b border-cream/10 last:border-0">
                <div className="flex-shrink-0">
                  <span className="text-sm font-mono text-terracotta font-semibold">
                    {String(i + 1).padStart(2, "0")}
                  </span>
                </div>
                <div className="flex-1">
                  <h3 className="text-lg md:text-xl font-semibold text-cream group-hover:text-terracotta transition-colors duration-300">
                    {phase.name}
                  </h3>
                  <p className="mt-1 text-sm text-cream/50">
                    {phase.description}
                  </p>
                </div>
                <div className="hidden md:block flex-shrink-0 max-w-[200px]">
                  <p className="text-xs font-mono text-terracotta/70">
                    {phase.intervention}
                  </p>
                </div>
              </div>
            </FadeIn>
          ))}
        </div>
      </div>
    </SectionWrapper>
  );
}

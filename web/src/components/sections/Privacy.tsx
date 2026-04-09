"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { Badge } from "@/components/ui/Badge";
import { FadeIn } from "@/components/animations/FadeIn";
import {
  StaggerReveal,
  StaggerItem,
} from "@/components/animations/StaggerReveal";
import { TECH_STACK } from "@/lib/constants";

export function Privacy() {
  return (
    <SectionWrapper id="privacy" className="bg-cream-dark">
      <div className="max-w-4xl mx-auto">
        <SectionHeading
          eyebrow="Privacy First"
          title="All Data Stays on Your Machine"
          description="No cloud, no telemetry, no tracking. VibeCheck stores behavioral metadata locally in SQLite. We never see your code, your prompts, or your data."
        />

        {/* Architecture diagram (text-based) */}
        <FadeIn delay={0.2}>
          <div className="mt-12 p-8 rounded-xl border border-border bg-cream">
            <div className="flex flex-col md:flex-row items-center justify-center gap-6 md:gap-12 text-center">
              <div className="p-4">
                <p className="text-sm font-mono font-semibold text-navy">
                  Your Computer
                </p>
                <p className="mt-1 text-xs text-navy-muted">
                  Window events, app names, timestamps
                </p>
              </div>
              <div className="text-2xl font-mono text-terracotta hidden md:block">
                &rarr;
              </div>
              <div className="text-2xl font-mono text-terracotta md:hidden">
                &darr;
              </div>
              <div className="p-4 border border-terracotta/30 rounded-lg">
                <p className="text-sm font-mono font-semibold text-terracotta">
                  VibeCheck
                </p>
                <p className="mt-1 text-xs text-navy-muted">
                  Classify, aggregate, analyze
                </p>
              </div>
              <div className="text-2xl font-mono text-terracotta hidden md:block">
                &rarr;
              </div>
              <div className="text-2xl font-mono text-terracotta md:hidden">
                &darr;
              </div>
              <div className="p-4">
                <p className="text-sm font-mono font-semibold text-navy">
                  SQLite (local)
                </p>
                <p className="mt-1 text-xs text-navy-muted">
                  ~/Library/Application Support/
                </p>
              </div>
            </div>
          </div>
        </FadeIn>

        <StaggerReveal
          className="mt-10 flex flex-wrap justify-center gap-3"
          staggerDelay={0.06}
        >
          {TECH_STACK.map((tech) => (
            <StaggerItem key={tech}>
              <Badge>{tech}</Badge>
            </StaggerItem>
          ))}
        </StaggerReveal>
      </div>
    </SectionWrapper>
  );
}

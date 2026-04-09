"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { StatCard } from "@/components/ui/StatCard";
import {
  StaggerReveal,
  StaggerItem,
} from "@/components/animations/StaggerReveal";
import { STATS } from "@/lib/constants";

export function Problem() {
  return (
    <SectionWrapper id="problem">
      <div className="max-w-5xl mx-auto">
        <SectionHeading
          eyebrow="The Research"
          title="Vibe Coding Has a Dependency Problem"
          description="Our research across 142 developers reveals patterns that mirror behavioral addiction: time distortion, compulsive engagement, and spending escalation."
        />

        <StaggerReveal
          className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-12 md:gap-8"
          staggerDelay={0.15}
        >
          {STATS.map((stat, i) => (
            <StaggerItem key={i}>
              <StatCard
                value={stat.value}
                suffix={stat.suffix}
                label={stat.label}
                citation={stat.citation}
                decimals={stat.suffix === "%" || stat.suffix === "h" ? 1 : 0}
              />
            </StaggerItem>
          ))}
        </StaggerReveal>
      </div>
    </SectionWrapper>
  );
}

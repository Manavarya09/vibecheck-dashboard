"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { FeatureCard } from "@/components/ui/FeatureCard";
import {
  StaggerReveal,
  StaggerItem,
} from "@/components/animations/StaggerReveal";
import { FadeIn } from "@/components/animations/FadeIn";
import { FEATURES } from "@/lib/constants";

export function ProductShowcase() {
  return (
    <SectionWrapper id="product">
      <div className="max-w-5xl mx-auto">
        <SectionHeading
          eyebrow="The Product"
          title="Your Coding Habits, Visualized"
          description="A system-tray app that quietly monitors your AI tool usage and surfaces the patterns you can't see yourself."
        />

        {/* Dashboard mockup placeholder */}
        <FadeIn delay={0.2}>
          <div className="mt-12 rounded-2xl border border-border bg-cream-dark p-8 md:p-12">
            <div className="aspect-video rounded-lg bg-navy/5 border border-navy/10 flex items-center justify-center">
              <div className="text-center">
                <p className="text-sm font-mono text-navy-muted">
                  VibeCheck Dashboard
                </p>
                <p className="mt-1 text-xs text-navy-muted/60">
                  Live session tracking, activity breakdown, wellness metrics
                </p>
              </div>
            </div>
          </div>
        </FadeIn>

        <StaggerReveal
          className="mt-12 grid grid-cols-1 md:grid-cols-2 gap-4"
          staggerDelay={0.1}
        >
          {FEATURES.map((feature, i) => (
            <StaggerItem key={i}>
              <FeatureCard
                title={feature.title}
                description={feature.description}
              />
            </StaggerItem>
          ))}
        </StaggerReveal>
      </div>
    </SectionWrapper>
  );
}

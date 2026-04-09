"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { FadeIn } from "@/components/animations/FadeIn";
import { STEPS } from "@/lib/constants";

export function HowItWorks() {
  return (
    <SectionWrapper id="how-it-works" className="bg-cream-dark">
      <div className="max-w-4xl mx-auto">
        <SectionHeading
          eyebrow="Getting Started"
          title="Three Steps to Self-Awareness"
        />

        <div className="mt-16 space-y-12">
          {STEPS.map((step, i) => (
            <FadeIn key={i} delay={i * 0.15}>
              <div className="flex gap-6 md:gap-10 items-start">
                <div className="flex-shrink-0">
                  <span className="text-4xl md:text-5xl font-bold font-mono text-terracotta/20">
                    {step.number}
                  </span>
                </div>
                <div className="pt-2">
                  <h3 className="text-xl md:text-2xl font-bold text-navy">
                    {step.title}
                  </h3>
                  <p className="mt-2 text-base text-navy-light leading-relaxed max-w-lg">
                    {step.description}
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

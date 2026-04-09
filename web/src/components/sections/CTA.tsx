"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { KineticHeadline } from "@/components/animations/KineticHeadline";
import { FadeIn } from "@/components/animations/FadeIn";
import { Button } from "@/components/ui/Button";

export function CTA() {
  return (
    <SectionWrapper className="bg-navy text-cream text-center">
      <div className="max-w-3xl mx-auto">
        <KineticHeadline
          text="Start Understanding Your AI Habits Today"
          tag="h2"
          className="text-3xl md:text-5xl font-bold tracking-tight text-cream leading-[1.1]"
        />

        <FadeIn delay={0.6}>
          <p className="mt-6 text-lg text-cream/60 max-w-xl mx-auto leading-relaxed">
            Free. Open source. No account required. All data stays on your machine.
          </p>
        </FadeIn>

        <FadeIn delay={0.9}>
          <div className="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
            <Button
              href="https://github.com/Manavarya09/vibecheck-dashboard/releases"
              variant="primary"
              className="bg-terracotta text-white hover:bg-terracotta-hover"
            >
              Download for macOS
            </Button>
            <Button
              href="https://github.com/Manavarya09/vibecheck-dashboard"
              variant="secondary"
              className="border-cream/20 text-cream hover:bg-cream/10"
            >
              View Source Code
            </Button>
          </div>
        </FadeIn>
      </div>
    </SectionWrapper>
  );
}

"use client";

import { SectionWrapper } from "@/components/layout/SectionWrapper";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { FadeIn } from "@/components/animations/FadeIn";

export function Research() {
  return (
    <SectionWrapper id="research">
      <div className="max-w-5xl mx-auto">
        <SectionHeading
          eyebrow="The Evidence"
          title="Built on Research, Not Assumptions"
        />

        <FadeIn delay={0.2}>
          <blockquote className="mt-12 max-w-3xl mx-auto">
            <p className="text-xl md:text-2xl text-navy leading-relaxed font-medium italic text-center">
              &ldquo;You remember the jackpots. You don&rsquo;t remember sitting
              there plugging tokens into the slot machine for two hours.&rdquo;
            </p>
            <footer className="mt-4 text-center">
              <cite className="text-sm font-mono text-navy-muted not-italic">
                Senior Engineering Director, via MIT Technology Review (Dec 2025)
              </cite>
            </footer>
          </blockquote>
        </FadeIn>

        <div className="mt-16 grid grid-cols-1 md:grid-cols-2 gap-8">
          <FadeIn delay={0.3} direction="up">
            <div className="p-6 rounded-xl border border-border">
              <h3 className="text-xs font-mono font-semibold uppercase tracking-widest text-terracotta mb-4">
                The Gambling Parallel
              </h3>
              <ul className="space-y-3 text-sm text-navy-light">
                <li className="flex gap-3">
                  <span className="text-terracotta font-mono text-xs mt-0.5 flex-shrink-0">--</span>
                  <span>Sunk-cost escalation mirrors &ldquo;I need to win back my losses&rdquo;</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-terracotta font-mono text-xs mt-0.5 flex-shrink-0">--</span>
                  <span>Near-miss code activates the same reward circuitry as slot machines</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-terracotta font-mono text-xs mt-0.5 flex-shrink-0">--</span>
                  <span>Rate limits hit at peak engagement -- worst time for rational decisions</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-terracotta font-mono text-xs mt-0.5 flex-shrink-0">--</span>
                  <span>The productivity alibi: &ldquo;I&rsquo;m not addicted, I&rsquo;m being productive&rdquo;</span>
                </li>
              </ul>
            </div>
          </FadeIn>

          <FadeIn delay={0.4} direction="up">
            <div className="p-6 rounded-xl border border-border">
              <h3 className="text-xs font-mono font-semibold uppercase tracking-widest text-terracotta mb-4">
                Key Papers
              </h3>
              <ul className="space-y-3 text-sm text-navy-light">
                <li className="flex gap-3">
                  <span className="text-navy-muted font-mono text-xs mt-0.5 flex-shrink-0">2025</span>
                  <span>Generative AI Addiction Syndrome (Ciudad-Fernandez et al.)</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-navy-muted font-mono text-xs mt-0.5 flex-shrink-0">2025</span>
                  <span>Dark Addiction Patterns of AI Chatbot Interfaces (CHI)</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-navy-muted font-mono text-xs mt-0.5 flex-shrink-0">2025</span>
                  <span>Vibe Coding as Material Disengagement (Sarkar, Microsoft)</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-navy-muted font-mono text-xs mt-0.5 flex-shrink-0">2025</span>
                  <span>AI Makes Developers 19% Slower (METR / Stanford)</span>
                </li>
              </ul>
            </div>
          </FadeIn>
        </div>
      </div>
    </SectionWrapper>
  );
}

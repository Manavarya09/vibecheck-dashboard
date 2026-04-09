"use client";

import { motion } from "framer-motion";
import { KineticHeadline } from "@/components/animations/KineticHeadline";
import { FadeIn } from "@/components/animations/FadeIn";
import { Button } from "@/components/ui/Button";
import { HERO_HEADLINE, HERO_SUBTEXT } from "@/lib/constants";

export function Hero() {
  return (
    <section className="relative min-h-screen flex flex-col items-center justify-center px-6 overflow-hidden">
      {/* Gradient background */}
      <div className="absolute inset-0 bg-gradient-to-b from-cream via-cream to-cream-dark" />

      {/* Subtle grid pattern */}
      <div
        className="absolute inset-0 opacity-[0.03]"
        style={{
          backgroundImage:
            "linear-gradient(var(--color-navy) 1px, transparent 1px), linear-gradient(90deg, var(--color-navy) 1px, transparent 1px)",
          backgroundSize: "60px 60px",
        }}
      />

      <div className="relative z-10 max-w-5xl mx-auto text-center">
        <FadeIn delay={0}>
          <p className="text-xs font-mono font-semibold uppercase tracking-[0.2em] text-terracotta mb-6">
            Developer Wellness Platform
          </p>
        </FadeIn>

        <KineticHeadline
          text={HERO_HEADLINE}
          className="text-5xl md:text-7xl lg:text-8xl font-bold tracking-tight text-navy leading-[1.05]"
          delay={0.3}
        />

        <FadeIn delay={1.2} duration={0.8}>
          <p className="mt-8 text-lg md:text-xl text-navy-light max-w-2xl mx-auto leading-relaxed">
            {HERO_SUBTEXT}
          </p>
        </FadeIn>

        <FadeIn delay={1.6} duration={0.6}>
          <div className="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
            <Button
              href="https://github.com/Manavarya09/vibecheck-dashboard/releases"
              variant="primary"
            >
              Download for macOS
            </Button>
            <Button
              href="https://github.com/Manavarya09/vibecheck-dashboard"
              variant="secondary"
            >
              View on GitHub
            </Button>
          </div>
        </FadeIn>
      </div>

      {/* Scroll indicator */}
      <motion.div
        className="absolute bottom-8 left-1/2 -translate-x-1/2"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 2.2, duration: 0.8 }}
      >
        <motion.p
          className="text-xs font-mono text-navy-muted tracking-widest uppercase"
          animate={{ y: [0, 6, 0] }}
          transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
        >
          scroll
        </motion.p>
      </motion.div>
    </section>
  );
}

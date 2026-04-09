export default function Home() {
  return (
    <main>
      <section className="min-h-screen flex flex-col items-center justify-center px-6">
        <h1 className="text-5xl md:text-7xl font-bold tracking-tight text-navy text-center max-w-4xl leading-[1.1]">
          Screen Time for the Age of Vibe Coding
        </h1>
        <p className="mt-6 text-lg md:text-xl text-navy-light text-center max-w-2xl leading-relaxed">
          The first developer wellness platform purpose-built for AI-assisted
          programming. Track your habits, detect unhealthy patterns, and build a
          better relationship with your AI tools.
        </p>
        <div className="mt-10 flex gap-4">
          <a
            href="https://github.com/Manavarya09/vibecheck-dashboard/releases"
            className="px-6 py-3 bg-terracotta text-white font-semibold rounded-lg hover:bg-terracotta-hover transition-colors"
          >
            Download for macOS
          </a>
          <a
            href="https://github.com/Manavarya09/vibecheck-dashboard"
            className="px-6 py-3 border border-navy/20 text-navy font-semibold rounded-lg hover:bg-cream-dark transition-colors"
          >
            View on GitHub
          </a>
        </div>
      </section>
    </main>
  );
}

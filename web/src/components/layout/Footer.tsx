import { FadeIn } from "@/components/animations/FadeIn";
import { GITHUB_REPO } from "@/lib/constants";

export function Footer() {
  return (
    <footer className="py-16 px-6 border-t border-border" role="contentinfo">
      <div className="max-w-5xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-12">
          <div>
            <p className="text-lg font-bold text-navy">VibeCheck</p>
            <p className="mt-2 text-sm text-navy-light leading-relaxed">
              Developer wellness for the age of AI-assisted programming. Open source, privacy-first, research-backed.
            </p>
          </div>

          <div>
            <p className="text-xs font-mono font-semibold uppercase tracking-widest text-navy-muted mb-4">
              Links
            </p>
            <ul className="space-y-2 text-sm text-navy-light">
              <li>
                <a href={`https://github.com/${GITHUB_REPO}`} className="hover:text-terracotta transition-colors">
                  GitHub Repository
                </a>
              </li>
              <li>
                <a href={`https://github.com/${GITHUB_REPO}/releases`} className="hover:text-terracotta transition-colors">
                  Download for macOS
                </a>
              </li>
              <li>
                <a href={`https://github.com/${GITHUB_REPO}/issues`} className="hover:text-terracotta transition-colors">
                  Report an Issue
                </a>
              </li>
            </ul>
          </div>

          <div>
            <p className="text-xs font-mono font-semibold uppercase tracking-widest text-navy-muted mb-4">
              Author
            </p>
            <p className="text-sm text-navy-light">Manav Arya Singh</p>
            <p className="text-sm text-navy-muted">Department of Computer Science</p>
            <p className="text-sm text-navy-muted">BITS Pilani, Dubai Campus</p>
          </div>
        </div>

        <div className="mt-12 pt-6 border-t border-border/50">
          <p className="text-xs text-navy-muted text-center font-mono">
            Built with VibeCheck tracking my own AI coding habits. MIT License.
          </p>
        </div>
      </div>
    </footer>
  );
}

import { Button } from "@/components/ui/Button";

export default function NotFound() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center px-6 bg-cream">
      <p className="text-8xl font-bold font-mono text-terracotta/20">404</p>
      <h1 className="mt-4 text-2xl font-bold text-navy">Page not found</h1>
      <p className="mt-2 text-navy-light">
        This page doesn&apos;t exist. You might be in an AI-Induced Flow Loop.
      </p>
      <div className="mt-8">
        <Button href="/" variant="primary">
          Back to Home
        </Button>
      </div>
    </div>
  );
}

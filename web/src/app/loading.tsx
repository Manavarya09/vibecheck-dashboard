export default function Loading() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-cream">
      <div className="flex flex-col items-center gap-4">
        <div className="w-8 h-8 border-2 border-terracotta/20 border-t-terracotta rounded-full animate-spin" />
        <p className="text-sm font-mono text-navy-muted">Loading</p>
      </div>
    </div>
  );
}

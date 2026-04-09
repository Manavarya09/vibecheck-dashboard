interface FeatureCardProps {
  title: string;
  description: string;
}

export function FeatureCard({ title, description }: FeatureCardProps) {
  return (
    <div className="p-6 rounded-xl border border-border bg-cream hover:bg-cream-dark transition-colors duration-200">
      <h3 className="text-base font-semibold text-navy">{title}</h3>
      <p className="mt-2 text-sm text-navy-light leading-relaxed">
        {description}
      </p>
    </div>
  );
}

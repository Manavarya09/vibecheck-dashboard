interface CitationProps {
  text: string;
}

export function Citation({ text }: CitationProps) {
  return (
    <span className="block mt-2 text-xs font-mono text-navy-muted">{text}</span>
  );
}

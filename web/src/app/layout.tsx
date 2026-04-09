import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
});

export const metadata: Metadata = {
  title: "VibeCheck -- Developer Wellness in the Age of AI",
  description:
    "The first developer wellness platform purpose-built for AI-assisted programming. Track your vibe coding habits, detect AI-Induced Flow Loops, and build healthier relationships with AI tools.",
  openGraph: {
    title: "VibeCheck -- Developer Wellness in the Age of AI",
    description:
      "Screen Time for the age of vibe coding. Track, understand, and improve your AI coding habits.",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${inter.variable} h-full antialiased`}>
      <body className="min-h-full bg-cream text-navy font-sans">{children}</body>
    </html>
  );
}

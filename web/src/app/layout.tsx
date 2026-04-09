import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Header } from "@/components/layout/Header";
import { CustomCursor } from "@/components/layout/CustomCursor";
import { ScrollProgress } from "@/components/layout/ScrollProgress";
import { Footer } from "@/components/layout/Footer";

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
    url: "https://vibecheck.dev",
    siteName: "VibeCheck",
    locale: "en_US",
  },
  twitter: {
    card: "summary_large_image",
    title: "VibeCheck -- Developer Wellness in the Age of AI",
    description: "Screen Time for the age of vibe coding.",
  },
  metadataBase: new URL("https://vibecheck.dev"),
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${inter.variable} h-full antialiased`}>
      <body className="min-h-full bg-cream text-navy font-sans">
          <ScrollProgress />
          <CustomCursor />
          <Header />
          {children}
          <Footer />
        </body>
    </html>
  );
}

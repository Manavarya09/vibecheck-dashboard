import { Hero } from "@/components/sections/Hero";
import { Problem } from "@/components/sections/Problem";
import { AiflFramework } from "@/components/sections/AiflFramework";
import { ProductShowcase } from "@/components/sections/ProductShowcase";
import { HowItWorks } from "@/components/sections/HowItWorks";
import { Research } from "@/components/sections/Research";
import { Privacy } from "@/components/sections/Privacy";

export default function Home() {
  return (
    <main>
      <Hero />
      <Problem />
      <AiflFramework />
      <ProductShowcase />
      <HowItWorks />
      <Research />
      <Privacy />
    </main>
  );
}

import { Hero } from "@/components/sections/Hero";
import { Problem } from "@/components/sections/Problem";
import { AiflFramework } from "@/components/sections/AiflFramework";
import { ProductShowcase } from "@/components/sections/ProductShowcase";

export default function Home() {
  return (
    <main>
      <Hero />
      <Problem />
      <AiflFramework />
      <ProductShowcase />
    </main>
  );
}

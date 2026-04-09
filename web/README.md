# VibeCheck Landing Page

The public-facing website for VibeCheck at vibecheck.dev.

## Stack

- Next.js 16 (App Router)
- Tailwind CSS v4
- Framer Motion (scroll animations)
- TypeScript

## Development

```sh
cd web
npm install
npm run dev
```

Open http://localhost:3000.

## Build

```sh
npm run build
```

## Deploy

Connected to Vercel. Push to `main` triggers auto-deploy.

## Architecture

Single-page design with 8 sections:

1. Hero -- kinetic headline, 3D background (planned), CTAs
2. Problem -- animated research statistics
3. AIFL Framework -- six-phase flow loop diagram
4. Product Showcase -- feature cards and mockup
5. How It Works -- three-step flow
6. Research -- quotes and citations
7. Privacy -- local-only architecture
8. CTA + Footer

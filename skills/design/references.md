# External Design References

## When to Use

When building components and the active design set doesn't have a pattern for the requested component type, consult these references for inspiration. Adapt patterns to the active design set's tokens and rules.

## Component Libraries

### shadcn/ui
- **URL**: https://ui.shadcn.com/docs/components
- **Strength**: Component architecture, CVA variant patterns, OKLCh color system, accessibility
- **Use for**: Button variants, form components, data tables, command palette, dialog
- **Key specs**: `--radius: 0.625rem (10px)`, Zinc palette, `disabled:opacity-50 disabled:pointer-events-none` pattern
- **Adapt**: React → Svelte 5, CVA → object map, Radix → native ARIA

### React Bits (135+ components)
- **URL**: https://reactbits.dev/
- **GitHub**: https://github.com/DavidHDev/react-bits
- **Strength**: Animation patterns, micro-interactions, visual effects, mouse-tracking

#### Component Catalog by Category

**Text Animations** (22 components):
BlurText, SplitText, ShinyText, GradientText, GlitchText, DecryptedText, ScrambleText, ScrollReveal, ScrollVelocity, TrueFocus, CountUp, CircularText, FuzzyText, RotatingText, ScrollFloat, ASCIIText, VariableProximity, TextPressure, FallingText, WaveText, TypingAnimation, SpringText

**Cursor & Interaction Effects** (14 components):
SplashCursor (WebGL fluid), BlobCursor, FollowCursor, PixelTrail, Magnet, MagnetLines, GlareHover, ShapeBlur, GradualBlur, AnimatedContent, PixelTransition, Crosshair, Noise, Ripple

**UI Components** (17 components):
AnimatedList, Dock, SpotlightCard, TiltedCard, GlassSurface, Carousel, CircularGallery, CardSwap, ElasticSlider, ChromaGrid, PillNav, ModelViewer, Stack, Masonry, InfiniteScroll, Accordion, Stepper

**Backgrounds** (12 components):
Aurora, Beams, Particles, Hyperspeed, Silk, LiquidEther, Dither, Iridescence, LetterGlitch, Lightning, LightRays, Orb

#### Components by Technique (for Svelte 5 translation)

**Pure CSS (no JS library)** — copy directly to `<style>`:
- ShinyText: `background-clip: text`, 200% size, `animation: shine 3s linear infinite`
- GradientText: `background-clip: text`, 400% size, `animation: gradient-shift 8s ease infinite`
- GlitchText: `clip-path: inset()` + `::before`/`::after` pseudo-elements, RGB separation `text-shadow: 2px 0 red`
- CircularText: Per-character `transform: rotate(calc(var(--angle) * var(--index)))`, container rotation keyframe

**Mouse-tracking** — `onmousemove` → `$state` → `style:` directive:
- SpotlightCard: `radial-gradient(circle at ${x}px ${y}px, rgba(255,255,255,0.15), transparent 80%)`
- TiltedCard: `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`, maxTilt 15deg, 400ms ease-out
- GlareHover: 300% radial gradient, 0.3 opacity, 800ms ease transition
- VariableProximity: `font-variation-settings` interpolated by cursor distance, falloff: linear/exponential/gaussian

**Canvas 2D** — `bind:this` + `requestAnimationFrame` in `$effect`:
- FuzzyText: `getImageData` → horizontal slice shifting → `putImageData`, baseIntensity/hoverIntensity props
- ASCIIText: Character-mapped pixel rendering

**WebGL** — Framework-agnostic shader code, wrap in Svelte component:
- SplashCursor: Fluid simulation, SIM_RESOLUTION 128, DYE_RESOLUTION 1024, CURL 30, SPLAT_RADIUS 0.25
- Iridescence: Iridescent shimmer shader

**IntersectionObserver** — `$effect` with observer setup/cleanup:
- BlurText: `filter: blur(10px)` → `blur(0)`, 0.5s, `cubic-bezier(0, 0.71, 0.2, 1.01)`
- ScrollReveal, ScrollFloat

**Spring physics** — `svelte/motion` `spring()` replacement for Framer Motion:
- Dock: `stiffness: 150, damping: 12, mass: 0.1` (snappy)
- AnimatedList: `stiffness: 500, damping: 30` (quick settle)

**JS interval** — `$effect` with `setInterval` + `$state` array:
- DecryptedText: Character scramble, speed 60ms, maxIterations 15, sequential mode

### Tailwind UI
- **URL**: https://tailwindui.com/components
- **Strength**: Production-ready layout patterns, responsive design
- **Use for**: Page layouts, navigation, marketing sections, application shells
- **Adapt**: React/Vue → Svelte 5, Headless UI → native Svelte

## Design System Documentation

### Apple Human Interface Guidelines
- **URL**: https://developer.apple.com/design/human-interface-guidelines/
- **Key specs sourced**: 8pt grid, 44pt touch targets, Dynamic Type scale (34px Large Title → 11px Caption 2), Core Animation easing `cubic-bezier(0.25, 0.1, 0.25, 1.0)` at 250ms default
- **Use for**: Touch target sizing, focus management, motion principles, hierarchy

### GitHub Primer
- **URL**: https://primer.style/
- **Primitives**: https://github.com/primer/primitives
- **Key specs sourced**: Blue-tinted gray scale (#F6F8FA→#1B1F24), system font stack, .f00-f6 type scale (40-12px), base-8 spacing, shadow tokens (resting/floating), 32px default button height, 4/6/8px border radius utilities, 6 color modes (light, dark, dimmed, high contrast, protanopia, tritanopia)
- **Use for**: Dense UI, developer tools, data-dense layouts, tree views

### Vercel Geist
- **URL**: https://vercel.com/geist/introduction
- **Font**: https://vercel.com/font
- **Key specs sourced**: Pure #000/#FFF endpoints, accent scale (#111→#fafafa), brand blue #0070F3, Geist Sans/Mono variable fonts (100-900 weight), borders-over-shadows, semantic colors (--geist-cyan #79FFE1, --geist-purple #F81CE5)
- **Use for**: Dark-first design, hero sections, metric cards, gradient backgrounds

### Stripe
- **URL**: https://stripe.com/docs/appearance
- **Key specs sourced**: Brand #635BFF, dark navy #0A2540, bg #F6F9FC, text #30313d, purple-tinted shadows from CodePen mixin (5 levels), 4px default border radius, body 18px weight 300, Inter font family
- **Use for**: Warm premium UI, onboarding flows, form design, card layouts

### Linear
- **URL**: https://linear.app
- **Key specs sourced**: LCH color space for dark mode elevation, Inter Display headings, weight 800 heroes / 600 labels / 400 body, text #F7F8F8 / #95A2B3 (from typ.io inspection)
- **Use for**: Dark mode patterns, minimal SaaS UI, keyboard-driven interfaces

## Fetch Protocol

When referencing external sources:
1. Use `WebFetch` to retrieve the relevant page
2. Extract the **PATTERN** — structure, layout, interaction model
3. **Never copy framework-specific code** — translate to Svelte 5
4. **Apply active design set tokens** — colors, spacing, radius, shadows from the set file
5. **Ensure accessibility** — add ARIA, keyboard nav, focus management
6. **Test both themes** — verify in light and dark mode
7. **Respect the set's animation timing** — don't bring foreign timing values

## Pattern Translation Guide

| Source Pattern | Svelte 5 Equivalent |
|----------------|---------------------|
| React `useState` | `$state()` |
| React `useMemo` | `$derived()` |
| React `useEffect` | `$effect()` |
| React props | `$props()` with `interface Props` |
| React `children` | `Snippet` from `'svelte'` |
| React `className` | `class` attribute |
| React `onClick` | `onclick` (lowercase) |
| React conditional render | `{#if}...{/if}` |
| React `.map()` | `{#each items as item}...{/each}` |
| React portal | Svelte `<svelte:body>` or manual mount |
| React context | `setContext`/`getContext` with Symbol key |
| framer-motion `useSpring` | `svelte/motion` `spring()` |
| framer-motion `AnimatePresence` | `{#if}` + `transition:` directive |
| framer-motion `layout` | `animate:flip` from `svelte/animate` |
| Radix primitives | Native HTML + ARIA attributes |
| CSS Modules | Svelte `<style>` (auto-scoped) |
| Tailwind `cn()` | Template literal with conditional classes |

# Bold Design Set

Vercel Geist inspired. Dark-first, high contrast, Swiss design minimalism. Built for marketing, dashboards, dev tools. "Simplicity, minimalism, and speed."

**Sources**: [Geist UI Colors](https://geist-ui.dev/en-us/guide/colors), [Geist UI Themes](https://geist-ui.dev/en-us/guide/themes), [Vercel Geist Colors](https://vercel.com/geist/colors), [Vercel Font](https://vercel.com/font), [geist-org/themes](https://github.com/geist-org/themes), [Geist Design System Figma](https://www.figma.com/community/file/1330020847221146106), [React Bits](https://www.reactbits.dev/)

## Color Tokens

### Dark Mode (Default — dark-first)

From [Geist UI Themes](https://geist-ui.dev/en-us/guide/themes) — dark palette (inverted accent scale):

```css
/* Geist dark theme — documented in geist-ui.dev/en-us/guide/themes */
--background: #000000;         /* palette.background */
--background-alt: #111111;     /* Geist palette.accents_1 */
--background-elevated: #1a1a1a;
--text-primary: #ffffff;       /* Geist palette.foreground */
--text-secondary: #999999;     /* Geist palette.accents_6 */
--text-muted: #666666;         /* Geist palette.accents_4 */
--border-color: #333333;       /* Geist palette.accents_2 */
--border-color-light: #444444;
--accent: #0070F3;             /* Vercel brand blue (Blue Ribbon) — vercel.com/geist/colors */
--accent-hover: #0060d0;
--accent-active: #0050b0;
--hover-overlay: rgba(255, 255, 255, 0.06);
--active-overlay: rgba(255, 255, 255, 0.1);
--focus-ring: #0070F3;
--card-bg: #111111;
--card-border: #333333;
--card-shadow: 0 0 0 1px var(--border-color);
--card-shadow-hover: 0 0 0 1px var(--text-secondary);
```

### Light Mode

From [Geist UI Themes](https://geist-ui.dev/en-us/guide/themes) — light palette:

```css
/* Geist light theme — documented in geist-ui.dev/en-us/guide/colors */
--background: #ffffff;         /* palette.background */
--background-alt: #fafafa;     /* Geist palette.accents_1 */
--background-elevated: #ffffff;
--text-primary: #000000;       /* Geist palette.foreground */
--text-secondary: #444444;     /* Geist palette.accents_6 */
--text-muted: #999999;         /* Geist palette.accents_3 */
--border-color: #eaeaea;       /* Geist palette.accents_2 */
--border-color-light: #f4f4f5;
--accent: #0070F3;
--accent-hover: #0060d0;
--accent-active: #0050b0;
--hover-overlay: rgba(0, 0, 0, 0.04);
--active-overlay: rgba(0, 0, 0, 0.08);
--focus-ring: #0070F3;
--card-bg: #ffffff;
--card-border: #eaeaea;
--card-shadow: 0 0 0 1px var(--border-color);
--card-shadow-hover: 0 0 0 1px var(--text-secondary);
```

### Semantic / Accent Colors

From [Vercel Geist Colors](https://vercel.com/geist/colors) and [geist-org/themes](https://github.com/geist-org/themes):

```css
/* Semantic status colors — universal conventions */
--color-success-500: #22c55e;  /* green — universal convention (dark: #4ade80) */
--color-warning-500: #F49B0B;  /* amber (dark: #facc15) */
--color-error-500: #EE0000;    /* red (dark: #f87171) */

/* Geist accent colors — decorative/extension use */
--geist-cyan: #79FFE1;
--geist-purple: #F81CE5;
--geist-violet: #7928CA;
--geist-alert: #FF0080;
--geist-warning: #F49B0B;
```

### Core Brand Neutrals

From [Mobbin Vercel brand analysis](https://mobbin.com/colors/brand/vercel):

```css
/* Cod Gray (dark base) */ #171717
/* Alabaster (light base) */ #FAFAFA
```

## Typography

From [Vercel Font](https://vercel.com/font) — "a typeface made for developers and designers, embodying Vercel's design principles of simplicity, minimalism, and speed, drawing inspiration from the renowned Swiss design movement."

- **Display/body**: `'Geist Sans', -apple-system, BlinkMacSystemFont, sans-serif` (variable font, weights 100-900)
- **Code/data**: `'Geist Mono', 'SF Mono', monospace` (variable font, weights 100-900)

### Type Scale

From [Vercel Geist Typography](https://vercel.com/geist/typography) — uses Tailwind class equivalents:

| Role | Size | Rem | Weight | Line Height |
|------|------|-----|--------|-------------|
| Caption | 12px | 0.75rem | 400 | 1.5 |
| Small body | 14px | 0.875rem | 400 | 1.5 |
| Body | 16px | 1rem | 400 | 1.5 |
| Large body | 18px | 1.125rem | 400 | 1.5 |
| Heading 3 | 20px | 1.25rem | 600 | 1.25 |
| Heading 2 | 24px | 1.5rem | 600 | 1.25 |
| Heading 1 | 32px | 2rem | 700 | 1.15 |
| Hero | 48px+ | 3rem+ | 800 | 1.0 |

- **Letter spacing**: Tighten on large text: -0.02em on headings, -0.04em on hero
- **Weight contrast**: 400 body / 600 headings / 700-800 hero (Geist Sans supports full 100-900 range)

## Spacing

From Geist CSS custom properties, aligned with Tailwind 4px base:

```css
--geist-gap: 16pt;            /* Standard component gap */
--geist-page-margin: 24pt;    /* Page-level margin */
```

- **Base unit**: 4px grid (Tailwind-aligned)
- **Hero padding**: 5rem-8rem (80-128px) vertical
- **Card padding**: 1.5rem (24px)
- **Button padding**: 0.625rem 1.5rem (10px 24px)
- **Gap scale**: gap-2 (8px), gap-4 (16px), gap-6 (24px), gap-8 (32px)

## Elevation

Geist favors **borders over shadows** — consistent with flat/minimal aesthetic. From [Vercel Geist Introduction](https://vercel.com/geist/introduction):

```css
/* Border-as-elevation: 1px borders via box-shadow for sub-pixel precision */
--card-shadow: 0 0 0 1px var(--border-color);
--card-shadow-hover: 0 0 0 1px var(--text-secondary);

/* Overlays only — modals, dropdowns */
--shadow-overlay: 0 8px 30px rgba(0, 0, 0, 0.12);
--shadow-overlay-dark: 0 8px 30px rgba(0, 0, 0, 0.6);
```

## Border Radius

From [Geist CSS properties](https://github.com/geist-org/themes):

```css
--geist-radius: 5px;  /* Default component radius */
/* Per-component: tight, crisp, angular aesthetic */
--radius-sm: 4px;     /* Badges, small elements */
--radius-md: 6px;     /* Buttons, inputs */
--radius-lg: 8px;     /* Cards */
--radius-xl: 12px;    /* Modals */
```

## Component Cheat Sheet

### Button — Primary (Geist style: inverted fg/bg)
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost' | 'error';
    size?: 'sm' | 'default' | 'lg';
    children: Snippet;
    onclick?: (e: MouseEvent) => void;
    disabled?: boolean;
  }
  let { variant = 'primary', size = 'default', children, onclick, disabled = false }: Props = $props();

  const variants = {
    primary: 'bg-[var(--text-primary)] text-[var(--background)] hover:opacity-90 active:scale-[0.97]',
    secondary: 'bg-[var(--background)] text-[var(--text-primary)] border border-[var(--border-color)] hover:border-[var(--text-muted)]',
    ghost: 'text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--hover-overlay)]',
    error: 'bg-[var(--color-error-500)] text-white hover:opacity-90 active:scale-[0.97]'
  };
  const sizes = {
    sm: 'h-8 px-3 text-xs font-medium rounded-[var(--radius-md)]',
    default: 'h-10 px-5 text-sm font-medium rounded-[var(--radius-md)]',
    lg: 'h-12 px-8 text-base font-semibold rounded-[var(--radius-md)]'
  };
</script>
<button class="inline-flex items-center justify-center transition-all duration-200
  focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--focus-ring)]
  disabled:pointer-events-none disabled:opacity-50
  {variants[variant]} {sizes[size]}" {disabled} {onclick}>
  {@render children()}
</button>
```

### Card (border-as-elevation, Geist pattern)
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { children: Snippet; hoverable?: boolean; class?: string; }
  let { children, hoverable = false, class: className = '' }: Props = $props();
</script>
<div class="rounded-[var(--radius-lg)] bg-[var(--background)] p-6
  shadow-[var(--card-shadow)]
  {hoverable ? 'transition-shadow duration-200 hover:shadow-[var(--card-shadow-hover)]' : ''}
  {className}">
  {@render children()}
</div>
```

### SpotlightCard (from [React Bits](https://www.reactbits.dev/))

Mouse-tracking radial gradient overlay. Source: React Bits SpotlightCard component.

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    children: Snippet;
    spotlightColor?: string;
    class?: string;
  }
  let { children, spotlightColor = 'rgba(255,255,255,0.15)', class: className = '' }: Props = $props();
  let x = $state(0);
  let y = $state(0);
  let opacity = $state(0);

  function handleMove(e: MouseEvent) {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    x = e.clientX - rect.left;
    y = e.clientY - rect.top;
  }
</script>
<div class="relative rounded-[var(--radius-lg)] bg-[var(--background)] shadow-[var(--card-shadow)] overflow-hidden {className}"
  onmousemove={handleMove}
  onmouseenter={() => opacity = 1}
  onmouseleave={() => opacity = 0}
  role="group">
  <div class="absolute inset-0 pointer-events-none transition-opacity duration-300"
    style:opacity
    style:background="radial-gradient(circle at {x}px {y}px, {spotlightColor}, transparent 80%)">
  </div>
  <div class="relative p-6">
    {@render children()}
  </div>
</div>
```

### TiltedCard (from [React Bits](https://www.reactbits.dev/))

3D perspective hover. Source: React Bits TiltedCard — perspective(1000px), maxTilt 15deg, 400ms ease-out.

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    children: Snippet;
    maxTilt?: number;
    perspective?: number;
    class?: string;
  }
  let { children, maxTilt = 15, perspective = 1000, class: className = '' }: Props = $props();
  let rotateX = $state(0);
  let rotateY = $state(0);
  let scale = $state(1);

  function handleMove(e: MouseEvent) {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;
    rotateX = ((mouseY - centerY) / centerY) * maxTilt;
    rotateY = ((mouseX - centerX) / centerX) * -maxTilt;
    scale = 1.05;
  }
  function handleLeave() { rotateX = 0; rotateY = 0; scale = 1; }
</script>
<div class="rounded-[var(--radius-lg)] bg-[var(--background)] shadow-[var(--card-shadow)] {className}"
  style="perspective: {perspective}px; transform-style: preserve-3d;
    transform: rotateX({rotateX}deg) rotateY({rotateY}deg) scale3d({scale},{scale},{scale});
    transition: transform 400ms ease-out;"
  onmousemove={handleMove}
  onmouseleave={handleLeave}
  role="group">
  <div class="p-6">
    {@render children()}
  </div>
</div>
```

### Hero Section (Geist dark aesthetic)
```svelte
<section class="relative py-20 lg:py-32 px-4 text-center overflow-hidden">
  <div class="absolute inset-0 bg-[var(--background)]"></div>
  <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(0,112,243,0.15),transparent_50%)]"></div>

  <div class="relative z-10 max-w-3xl mx-auto">
    <h1 class="text-4xl sm:text-5xl lg:text-6xl font-extrabold tracking-tight text-[var(--text-primary)] leading-[1.0]"
      style="letter-spacing: -0.04em;">
      <!-- Hero headline -->
    </h1>
    <p class="mt-6 text-lg text-[var(--text-secondary)] max-w-xl mx-auto leading-relaxed">
      <!-- Subtitle -->
    </p>
    <div class="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
      <!-- CTA buttons -->
    </div>
  </div>
</section>
```

### Metric Card
```svelte
<script lang="ts">
  import { TrendingUp, TrendingDown } from 'lucide-svelte';
  interface Props { label: string; value: string; change?: number; }
  let { label, value, change }: Props = $props();
  let trend = $derived(change && change > 0 ? 'up' : 'down');
</script>
<div class="rounded-[var(--radius-lg)] bg-[var(--background)] shadow-[var(--card-shadow)] p-5">
  <p class="text-xs font-medium uppercase tracking-wider text-[var(--text-muted)]">{label}</p>
  <div class="mt-2 flex items-baseline gap-3">
    <p class="text-3xl font-bold font-[tabular-nums] text-[var(--text-primary)]">{value}</p>
    {#if change !== undefined}
      <span class="inline-flex items-center gap-1 text-sm font-medium
        {trend === 'up' ? 'text-[var(--geist-cyan)]' : 'text-[var(--color-error-500)]'}">
        {#if trend === 'up'}<TrendingUp class="h-3.5 w-3.5" />{:else}<TrendingDown class="h-3.5 w-3.5" />{/if}
        {Math.abs(change)}%
      </span>
    {/if}
  </div>
</div>
```

### Badge
```svelte
<script lang="ts">
  type Variant = 'default' | 'success' | 'warning' | 'error' | 'cyan' | 'violet';
  interface Props { variant?: Variant; children: import('svelte').Snippet; }
  let { variant = 'default', children }: Props = $props();
  const classes: Record<Variant, string> = {
    default: 'bg-[var(--background-alt)] text-[var(--text-primary)] border-[var(--border-color)]',
    success: 'bg-[#22c55e]/10 text-[#22c55e] border-[#22c55e]/20',
    warning: 'bg-[#F49B0B]/10 text-[#F49B0B] border-[#F49B0B]/20',
    error: 'bg-[#EE0000]/10 text-[#EE0000] border-[#EE0000]/20',
    cyan: 'bg-[#79FFE1]/10 text-[#79FFE1] border-[#79FFE1]/20',
    violet: 'bg-[#7928CA]/10 text-[#7928CA] border-[#7928CA]/20'
  };
</script>
<span class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium {classes[variant]}">
  {@render children()}
</span>
```

### Navigation (transparent -> solid on scroll)
```svelte
<script lang="ts">
  let scrolled = $state(false);
  $effect(() => {
    const handler = () => { scrolled = window.scrollY > 20; };
    window.addEventListener('scroll', handler, { passive: true });
    return () => window.removeEventListener('scroll', handler);
  });
</script>
<header class="fixed top-0 inset-x-0 z-50 transition-all duration-200
  {scrolled
    ? 'bg-[var(--background)]/80 backdrop-blur-lg border-b border-[var(--border-color)]'
    : 'bg-transparent'}">
  <nav class="max-w-6xl mx-auto flex items-center justify-between px-6 h-16">
    <!-- Logo + nav items + CTA -->
  </nav>
</header>
```

## Animation

Geist transitions are fast and functional — aligned with "speed" design principle:

- **Micro-interactions**: 150-200ms ease or ease-in-out (Geist standard)
- **Content transitions**: 200-300ms ease-out
- **Hero entrances**: 400-500ms with stagger delays (100ms between elements)
- **Button feedback**: `transform: scale(0.97)` on `:active`
- **Card hover**: Border brightens (shadow swap), 200ms

### React Bits Animation Patterns (sourced values)

From [React Bits](https://www.reactbits.dev/) — documented spring/easing values:

| Pattern | Duration | Easing | Notes |
|---------|----------|--------|-------|
| SpotlightCard | 300ms | ease (opacity) | Mouse-tracking radial gradient |
| TiltedCard | 400ms | ease-out | perspective(1000px), maxTilt 15deg |
| GlareHover | 800ms | ease | 300% size, 0.3 opacity radial gradient |
| ShinyText | 3s | linear infinite | background-clip:text, 200% bg-size |
| BlurText reveal | 500ms | cubic-bezier(0, 0.71, 0.2, 1.01) | blur(10px) -> blur(0) |
| GlitchText | 500ms multiplier | step-based | clip-path + RGB separation (2px offset) |
| Dock magnification | spring | stiffness:150, damping:12, mass:0.1 | macOS-style magnification |
| AnimatedList | spring | stiffness:500, damping:30 | Staggered enter/exit |
| Gradient sweep | 8s | ease infinite | background-position animation |

### CSS Keyframe Examples (from React Bits source)

ShinyText shimmer (pure CSS):
```css
@keyframes shine {
  0% { background-position: 100%; }
  100% { background-position: -100%; }
}
.shiny-text {
  background-image: linear-gradient(120deg, rgba(255,255,255,0.4) 40%, rgba(255,255,255,1) 50%, rgba(255,255,255,0.4) 60%);
  background-size: 200% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: shine 3s linear infinite;
}
```

GlitchText RGB separation (pure CSS):
```css
.glitch::before, .glitch::after {
  content: attr(data-text);
  position: absolute;
  inset: 0;
}
.glitch::before { text-shadow: 2px 0 #FF0080; clip-path: inset(40% 0 20% 0); }
.glitch::after { text-shadow: -2px 0 #79FFE1; clip-path: inset(20% 0 40% 0); }
```

- **Reduced motion**: All animations disabled via `prefers-reduced-motion: reduce`

## Accessibility
- **Focus ring**: `outline: 2px solid var(--focus-ring); outline-offset: 2px` (Geist accent blue)
- **Touch targets**: 44px minimum
- **Contrast on dark**: #FFFFFF on #000000 = 21:1 (maximum)
- **Screen reader**: `.sr-only` utility class
- **High contrast**: `@media (forced-colors: active)` support
- **Color not sole indicator**: Icons + text for all status indicators

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]` (pure black/white), `bg-[var(--background-alt)]` (#111/#fafafa)
- **Text**: `text-[var(--text-primary)]`, `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: `shadow-[var(--card-shadow)]` — uses box-shadow for 1px borders (Geist pattern)
- **Hover**: Border brighten via shadow swap, or `hover:opacity-90` on filled buttons
- **Accent**: `text-[var(--accent)]` (#0070F3), semantic colors via `text-[var(--geist-cyan)]` etc.
- **Icon sizes**: `h-3.5 w-3.5` (inline), `h-4 w-4` (standard), `h-5 w-5` (nav)

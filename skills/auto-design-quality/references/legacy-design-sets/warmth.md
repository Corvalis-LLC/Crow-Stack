# Warmth Design Set

Stripe + Linear inspired. Consumer SaaS, collaborative tools, onboarding flows. Soft shadows, generous space, friendly personality. Approachable and premium without being corporate.

**Sources**: [Stripe Accessible Color Systems](https://stripe.com/blog/accessible-color-systems), [Stripe Appearance API](https://docs.stripe.com/elements/appearance-api), [Stripe Shadow Mixin](https://codepen.io/qbert/pen/XRjJKg), [Sohne on Stripe (typ.io)](https://typ.io/s/59wr), [Linear UI Redesign](https://linear.app/now/how-we-redesigned-the-linear-ui), [Inter UI on Linear (typ.io)](https://typ.io/s/2jmp)

## Color Tokens

Stripe uses **warm navy-blue tinted neutrals** (never pure gray). Their palette uses CIELAB (Lab) perceptually uniform color space for generating scales that predictably pass WCAG contrast ratios. Every gray has a slight blue undertone for a trustworthy, premium feel.

### Light Mode
```css
--background: #f6f9fc;  --background-alt: #ffffff;  --background-elevated: #ffffff;
/* Stripe light bg: #F6F9FC (Black Squeeze — very slight cool blue tint) */
--text-primary: #30313d;  --text-secondary: #525266;  --text-muted: #8792a2;
/* Stripe text: #30313d (documented in Appearance API — dark blue-gray, NOT pure black) */
--border-color: #e3e8ee;  --border-color-light: #f0f3f7;
--accent: #635bff;  --accent-hover: #5046e5;  --accent-active: #4338ca;
/* Stripe primary brand: #635BFF (Cornflower/Blurple — warm violet-leaning) */
--hover-overlay: rgba(99, 91, 255, 0.04);  --active-overlay: rgba(99, 91, 255, 0.08);
--focus-ring: #635bff;
--card-bg: #ffffff;  --card-border: #e3e8ee;
/* Stripe's signature: purple-tinted shadows using rgba(50,50,93,...) */
--card-shadow: 0 2px 5px rgba(50,50,93,0.09), 0 1px 2px rgba(0,0,0,0.07);
--card-shadow-hover: 0 4px 6px rgba(50,50,93,0.09), 0 1px 3px rgba(0,0,0,0.08);
--card-shadow-elevated: 0 15px 35px rgba(50,50,93,0.1), 0 5px 15px rgba(0,0,0,0.07);
```

### Dark Mode

Linear uses **LCH color space** for elevation — background, panels, dialogs, modals derive from the base color at different lightness levels. Theme system reduced to 3 variables: base color, accent color, contrast.

```css
--background: #0a2540;  --background-alt: #0f2d4e;  --background-elevated: #15395f;
/* Stripe dark navy: #0A2540 (Downriver) — not pure black */
--text-primary: #f7f8f8;  --text-secondary: #95a2b3;  --text-muted: #6b7c93;
/* Linear heading text: #F7F8F8, secondary: #95A2B3 (from typ.io inspection) */
--border-color: #1a3a5c;  --border-color-light: #234b73;
--accent: #818cf8;  --accent-hover: #a5b4fc;  --accent-active: #6366f1;
--hover-overlay: rgba(129, 140, 248, 0.06);  --active-overlay: rgba(129, 140, 248, 0.1);
--focus-ring: #818cf8;
--card-bg: #0f2d4e;  --card-border: #1a3a5c;
--card-shadow: 0 2px 8px rgba(0,0,0,0.3), 0 1px 2px rgba(0,0,0,0.2);
--card-shadow-hover: 0 4px 12px rgba(0,0,0,0.35), 0 2px 4px rgba(0,0,0,0.25);
```

### Semantic Colors
```css
--color-success-500: #10b981 / #34d399;
--color-warning-500: #f59e0b / #fbbf24;
--color-error-500: #df1b41 / #f87171;
/* Stripe danger: #df1b41 (documented in Appearance API) — differs from dadson default #dc2626 */
```

## Typography

Stripe uses **Sohne** (Klim Type Foundry, variable font) — described as "the memory of Akzidenz-Grotesk framed through the reality of Helvetica." We use Inter as the open-source substitute.

Linear uses **Inter Display** for headings, **Inter** for body. Linear's style: extreme weight contrast — 800 (extra bold) for heroes, 600 for labels, 400 for body.

- **Font**: `'Inter', system-ui, -apple-system, sans-serif`
- **Stripe body approach**: 18px at weight 300 (light) with generous 28px line height (~1.55x)
- **Stripe heading approach**: weight 500 (medium), NOT bold — refined, elegant
- **Linear heading approach**: weight 600-800, tight tracking — dramatic contrast

| Role | Size | Weight | Line Height | Source |
|------|------|--------|-------------|--------|
| Hero | 2.375rem (38px) | 500 | 1.26 | Stripe H2 hero (typ.io: 38px/500/48px) |
| H1 | 2rem (32px) | 600 | 1.25 | Linear-style bold heading |
| H2 | 1.5rem (24px) | 600 | 1.3 | |
| H3 | 1.125rem (18px) | 500 | 1.55 | Stripe section header (typ.io: 18px/500/28px) |
| Body | 1.125rem (18px) | 300 | 1.55 | Stripe body (typ.io: 18px/300/28px) |
| Small | 0.875rem (14px) | 500 | 1.5 | Stripe Elements base (documented: 14px/500) |
| Label | 0.75rem (12px) | 600 | 1.25 | Linear label (typ.io: 12px/600, uppercase, ls: 11px) |

- **Letter spacing**: -0.02em on headings >= 24px
- **Linear-style labels**: 12px, weight 600, uppercase, wide letter-spacing, color `--text-muted`

## Spacing

Stripe uses **2px as documented base unit** (Appearance API `spacingUnit`) but achieves generous spacing through larger element sizes. Body text at 18px is larger than most dashboards.

- **Base unit**: 8px (0.5rem) grid, generous interpretation
- **Grid**: 8, 12, 16, 24, 32, 48, 64, 80
- **Card padding**: 1.5rem (24px) — Stripe dashboard pattern
- **Button padding**: 0.625rem 1.25rem (10px 20px)
- **Input padding**: 0.75rem 1rem (12px 16px)
- **Section gaps**: 3rem-5rem (48-80px) — marketing pages use 48-80px
- **Dashboard gaps**: 1.5rem-2rem (24-32px)
- **Gap scale**: gap-3 (12px), gap-4 (16px), gap-6 (24px), gap-8 (32px)
- **Whitespace**: 50-70% — content breathes, generous margins

## Elevation

Stripe's **signature**: multi-layer shadows with purple-tinted `rgba(50,50,93,...)` alongside `rgba(0,0,0,...)`. This purple tint adds warmth — unique to Stripe.

```css
/* Extension: Stripe 5-level shadow system (above canonical) */
/* Sourced from Stripe shadow mixin (codepen.io/qbert/pen/XRjJKg) */
--shadow-1: 0 2px 5px rgba(50,50,93,0.09), 0 1px 2px rgba(0,0,0,0.07);    /* subtle */
--shadow-2: 0 4px 6px rgba(50,50,93,0.09), 0 1px 3px rgba(0,0,0,0.08);    /* default card */
--shadow-3: 0 1px 5px 0 rgba(0,0,0,0.07), 0 7px 17px 0 rgba(0,0,0,0.1);  /* medium */
--shadow-4: 0 15px 35px rgba(50,50,93,0.1), 0 5px 15px rgba(0,0,0,0.07);  /* elevated */
--shadow-5: 0 15px 35px rgba(50,50,93,0.15), 0 5px 15px rgba(0,0,0,0.1);  /* dramatic */
/* Stripe Elements tab: 0px 1px 1px rgba(0,0,0,0.03), 0px 3px 6px rgba(18,42,66,0.02) */
```
- **Hover lifts** — cards translate up + shadow upgrade (shadow-2 → shadow-4)
- **Shadows over borders** — minimal border use, rely on elevation
- **Linear approach**: Elevation through surface color (LCH lightness), not shadow

## Border Radius

Stripe documented default: **4px** (Appearance API `borderRadius`). Conservative rounding — friendly but not playful.

```css
--radius-sm: 0.25rem;  /* 4px - Stripe default */
--radius-md: 0.375rem; /* 6px - inputs, buttons */
--radius-lg: 0.5rem;   /* 8px - cards, panels */
--radius-xl: 0.75rem;  /* 12px - large cards, modals */
/* rounded-full for pills, avatars, toggles */
```

## Component Cheat Sheet

### Button — Primary
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost';
    size?: 'sm' | 'default' | 'lg';
    children: Snippet;
    onclick?: (e: MouseEvent) => void;
    disabled?: boolean;
  }
  let { variant = 'primary', size = 'default', children, onclick, disabled = false }: Props = $props();

  const variants = {
    primary: 'bg-[var(--accent)] text-white shadow-[var(--shadow-2)] hover:shadow-[var(--shadow-4)] active:scale-[0.98]',
    secondary: 'bg-[var(--background-alt)] text-[var(--text-primary)] border border-[var(--border-color)] shadow-[var(--shadow-1)] hover:shadow-[var(--shadow-2)] active:scale-[0.98]',
    ghost: 'text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--hover-overlay)]'
  };
  const sizes = {
    sm: 'h-9 px-3.5 text-sm font-medium rounded-[var(--radius-md)]',
    default: 'h-10 px-5 text-sm font-medium rounded-[var(--radius-md)]',
    lg: 'h-12 px-6 text-base font-medium rounded-[var(--radius-md)]'
  };
</script>
<button class="inline-flex items-center justify-center transition-all duration-200
  focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--focus-ring)]
  disabled:pointer-events-none disabled:opacity-50
  {variants[variant]} {sizes[size]}" {disabled} {onclick}>
  {@render children()}
</button>
```

### Card (with hover lift)
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { children: Snippet; hoverable?: boolean; class?: string; }
  let { children, hoverable = false, class: className = '' }: Props = $props();
</script>
<div class="rounded-[var(--radius-lg)] bg-[var(--card-bg)] p-6 shadow-[var(--shadow-2)]
  {hoverable ? 'transition-all duration-200 hover:shadow-[var(--shadow-4)] hover:-translate-y-0.5' : ''}
  {className}">
  {@render children()}
</div>
```

### Form Input
```svelte
<script lang="ts">
  interface Props { id?: string; label: string; value?: string; placeholder?: string; error?: string; hint?: string; }
  let { id = `field-${Math.random().toString(36).slice(2, 9)}`, label, value = $bindable(''), placeholder = '', error, hint }: Props = $props();
</script>
<div class="space-y-2">
  <label for={id} class="text-sm font-medium text-[var(--text-primary)]">{label}</label>
  <input {id} bind:value {placeholder}
    class="h-11 w-full rounded-[var(--radius-md)] border px-4 text-base transition-all duration-200
      {error ? 'border-[var(--color-error-500)] focus-visible:outline-[var(--color-error-500)]' : 'border-[var(--border-color)]'}
      bg-[var(--background-alt)] text-[var(--text-primary)] placeholder:text-[var(--text-muted)]
      focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[var(--focus-ring)]
      hover:border-[var(--border-color-light)]"
    aria-invalid={!!error}
    aria-describedby={error ? `${id}-error` : hint ? `${id}-hint` : undefined} />
  {#if error}
    <p id="{id}-error" class="text-sm text-[var(--color-error-500)]">{error}</p>
  {:else if hint}
    <p id="{id}-hint" class="text-sm text-[var(--text-muted)]">{hint}</p>
  {/if}
</div>
```

### Toast
```svelte
<script lang="ts">
  import { CheckCircle, AlertCircle, Info, X } from 'lucide-svelte';
  import { fly } from 'svelte/transition';
  type Variant = 'success' | 'error' | 'info';
  interface Props { message: string; variant?: Variant; onclose: () => void; }
  let { message, variant = 'info', onclose }: Props = $props();
  const icons = { success: CheckCircle, error: AlertCircle, info: Info };
  const Icon = $derived(icons[variant]);
</script>
<div class="flex items-center gap-3 rounded-[var(--radius-lg)] bg-[var(--card-bg)] p-4
  shadow-[var(--shadow-4)] border border-[var(--border-color)]"
  transition:fly={{ y: 20, duration: 250 }}
  role="alert">
  <Icon class="h-5 w-5 shrink-0
    {variant === 'success' ? 'text-[var(--color-success-500)]' : ''}
    {variant === 'error' ? 'text-[var(--color-error-500)]' : ''}
    {variant === 'info' ? 'text-[var(--accent)]' : ''}" />
  <p class="text-sm text-[var(--text-primary)] flex-1">{message}</p>
  <button onclick={onclose} class="rounded-full p-1 hover:bg-[var(--hover-overlay)] transition-colors duration-150"
    aria-label="Dismiss">
    <X class="h-4 w-4 text-[var(--text-muted)]" />
  </button>
</div>
```

### Empty State
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  import { TrendingUp } from 'lucide-svelte';
  interface Props { icon?: typeof TrendingUp; title: string; description: string; action?: Snippet; }
  let { icon: Icon, title, description, action }: Props = $props();
</script>
<div class="flex flex-col items-center justify-center py-16 px-4 text-center">
  {#if Icon}
    <div class="rounded-2xl bg-[rgba(99,91,255,0.06)] p-4 mb-5">
      <Icon class="h-8 w-8 text-[var(--accent)]" />
    </div>
  {/if}
  <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-1.5">{title}</h3>
  <p class="text-sm text-[var(--text-muted)] max-w-sm mb-6">{description}</p>
  {#if action}
    {@render action()}
  {/if}
</div>
```

### Navigation Sidebar
```svelte
<aside class="w-64 bg-[var(--background-alt)] border-r border-[var(--border-color)] p-3">
  <a class="flex items-center gap-3 px-3 py-2.5 rounded-[var(--radius-md)] text-sm font-medium
    text-[var(--text-secondary)] hover:bg-[var(--hover-overlay)] hover:text-[var(--text-primary)]
    transition-all duration-200">
    <Icon class="h-5 w-5 shrink-0" />
    <span>{label}</span>
  </a>
  <!-- Active state -->
  <a class="flex items-center gap-3 px-3 py-2.5 rounded-[var(--radius-md)] text-sm font-medium
    bg-[rgba(99,91,255,0.06)] text-[var(--accent)]">
    <Icon class="h-5 w-5 shrink-0" />
    <span>{label}</span>
  </a>
</aside>
```

## Animation

Stripe: under 500ms max, most 150-300ms, **custom cubic-bezier** curves (not built-in ease). Only animates `transform` and `opacity` (GPU-friendly). Uses `will-change` for GPU offloading. Linear: speed-first, 80-150ms for most interactions.

| Type | Duration | Easing |
|------|----------|--------|
| Micro-interactions | 150-200ms | custom `cubic-bezier` |
| Content transitions | 200-250ms | `ease-out` |
| Card hover lift | 200ms | `ease-out` |
| Layout changes | 300ms | `ease` |
| Slide transitions | 250ms | `fly({ y: 20 })` |
| Max | 400ms | — |

- **Button feedback**: `transform: scale(0.98)` on `:active`
- **Card hover**: `translateY(-2px)` + shadow upgrade (shadow-2 → shadow-4)
- **Only animate**: `transform` and `opacity` (Stripe performance rule)
- **Reduced motion**: All animations disabled via `prefers-reduced-motion: reduce`

## Accessibility
- **Focus ring**: `outline: 2px solid var(--focus-ring); outline-offset: 2px`
- **Touch targets**: 44px minimum everywhere
- **Input height**: 44px (h-11) — comfortable, senior-friendly
- **Screen reader**: `.sr-only` utility class
- **High contrast**: `@media (forced-colors: active)` support
- **ARIA patterns**: `aria-expanded`, `aria-haspopup`, `role="dialog"`, `role="alert"`

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]` (#f6f9fc warm), `bg-[var(--card-bg)]`, `bg-[var(--background-alt)]`
- **Text**: `text-[var(--text-primary)]` (#30313d blue-gray), `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: Minimal — prefer Stripe shadow elevation. When needed: `border-[var(--border-color)]`
- **Shadows**: `shadow-[var(--shadow-2)]` default, `shadow-[var(--shadow-4)]` hover/elevated
- **Hover**: `hover:-translate-y-0.5` + shadow upgrade
- **Radius**: `rounded-[var(--radius-md)]` (6px) default, `rounded-[var(--radius-lg)]` (8px) cards
- **Icon sizes**: `h-4 w-4` (inline), `h-5 w-5` (standard), `h-8 w-8` (empty states)

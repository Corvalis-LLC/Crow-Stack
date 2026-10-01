# Precision Design Set

Apple HIG + shadcn/ui inspired. Tight, monochrome, systematic. Data-dense apps, dev tools, professional dashboards. Maximum clarity with minimum decoration.

**Sources**: [Apple HIG Typography](https://developer.apple.com/design/human-interface-guidelines/typography), [Apple HIG Color](https://developer.apple.com/design/human-interface-guidelines/color), [Apple HIG Motion](https://developer.apple.com/design/human-interface-guidelines/motion), [shadcn/ui Theming](https://ui.shadcn.com/docs/theming), [shadcn/ui Button](https://ui.shadcn.com/docs/components/button)

## Color Tokens

shadcn/ui uses OKLCh color space (perceptually uniform). Convention: `--<name>` is background, `--<name>-foreground` is text on that background. Palette: Zinc (default neutral).

### Light Mode
```css
--background: #ffffff;  --background-alt: #fafafa;  --background-elevated: #ffffff;
--text-primary: #09090b;  --text-secondary: #71717a;  --text-muted: #a1a1aa;
--border-color: #e4e4e7;  --border-color-light: #f4f4f5;
--accent: #18181b;  --accent-hover: #27272a;  --accent-active: #3f3f46;  /* shadcn default: near-black primary */
--hover-overlay: rgba(0, 0, 0, 0.03);  --active-overlay: rgba(0, 0, 0, 0.06);
--focus-ring: #a1a1aa;  /* shadcn --ring: medium gray */
--card-bg: #ffffff;  --card-border: #e4e4e7;
--card-shadow: 0 1px 2px rgba(0,0,0,0.05);
--card-shadow-hover: 0 1px 3px rgba(0,0,0,0.08);
```

### Dark Mode
```css
--background: #09090b;  --background-alt: #18181b;  --background-elevated: #27272a;
--text-primary: #fafafa;  --text-secondary: #a1a1aa;  --text-muted: #71717a;
--border-color: rgba(255,255,255,0.1);  --border-color-light: rgba(255,255,255,0.15);
--accent: #e4e4e7;  --accent-hover: #fafafa;  --accent-active: #d4d4d8;
--hover-overlay: rgba(255, 255, 255, 0.04);  --active-overlay: rgba(255, 255, 255, 0.08);
--focus-ring: #71717a;
--card-bg: #18181b;  --card-border: rgba(255,255,255,0.1);
--card-shadow: 0 1px 2px rgba(0,0,0,0.4);
--card-shadow-hover: 0 1px 3px rgba(0,0,0,0.5);
```

### Semantic Colors
```css
--color-success-500: #22c55e / #4ade80;
--color-warning-500: #eab308 / #facc15;
--color-error-500: #ef4444 / #f87171;
```

## Typography

Apple HIG: SF Pro uses **SF Pro Display** at 20pt+ and **SF Pro Text** below 20pt. Recommended weights: Regular, Medium, Semibold, Bold only (avoid Ultralight/Thin/Light for readability).

- **Font**: `'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif`
- **Scale** (derived from Apple iOS Dynamic Type at "Large" content size):

| Role | Size | Weight | Line Height | Notes |
|------|------|--------|-------------|-------|
| Hero | 2.125rem (34px) | 400 | 1.15 | Apple Large Title |
| H1 | 1.75rem (28px) | 400 | 1.2 | Apple Title 1 |
| H2 | 1.375rem (22px) | 400 | 1.25 | Apple Title 2 |
| H3 | 1.25rem (20px) | 400 | 1.3 | Apple Title 3 — Display/Text threshold |
| Headline | 1.0625rem (17px) | 600 | 1.4 | Apple Headline (same size as Body, bolder) |
| Body | 1.0625rem (17px) | 400 | 1.5 | Apple Body — default reading text |
| Callout | 1rem (16px) | 400 | 1.5 | Apple Callout |
| Small | 0.875rem (14px) | 400 | 1.5 | shadcn text-sm — buttons, inputs |
| Caption | 0.75rem (12px) | 400 | 1.5 | Apple Caption 1 |
| Minimum | 0.6875rem (11px) | 400 | 1.4 | Apple Caption 2 — never go smaller |

- **Tabular numerals** everywhere — all numbers use `font-[tabular-nums]`
- **Letter spacing**: -0.003em on desktop headings (from Apple web CSS), 0.01em on uppercase labels
- **Table headers**: 0.75rem, font-weight 500, uppercase, tracking-wide

## Spacing

Apple HIG uses the **8pt grid system**. All values are multiples of 8, with 4pt sub-grid for tight layouts.

- **Grid**: 8, 16, 24, 32, 40, 48 — strict 8pt multiples
- **Card padding**: 1rem (16px)
- **Button padding**: 0.5rem 1rem (8px 16px) — shadcn default h-10 = 40px
- **Input padding**: 0.5rem 0.75rem (8px 12px)
- **Table cell padding**: 0.5rem 0.75rem (dense)
- **Gap scale**: gap-2 (8px), gap-3 (12px), gap-4 (16px), gap-6 (24px)
- **Whitespace**: 60-80% — generous around content, tight within groups
- **Apple web margins**: Desktop 92px top / 140px bottom, Mobile 45px top / 60px bottom

## Elevation

```css
--card-shadow: 0 1px 2px rgba(0,0,0,0.05);
```
- **0-2 levels only** — flat is default, shadow is subtle
- **Borders preferred** over shadows for separation (shadcn pattern)
- **No hover shadow changes** — use border-color change instead

## Border Radius

shadcn/ui default `--radius: 0.625rem` (10px). Applied consistently across all components via CSS variable.

```css
--radius-sm: 0.25rem;  /* 4px */
--radius-md: 0.375rem; /* 6px - shadcn rounded-md */
--radius-lg: 0.5rem;   /* 8px - cards, modals */
--radius: 0.625rem;    /* 10px - shadcn theme-level default */
```

## Component Cheat Sheet

### Button — CVA Pattern (from shadcn/ui source)

shadcn/ui button: base classes always applied, variants and sizes composed. `disabled:opacity-50 disabled:pointer-events-none` universal. Focus: `focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2`.

Sizes: `default` h-10 (40px), `sm` h-9 (36px), `lg` h-11 (44px — Apple 44pt touch target).

```svelte
<script lang="ts">
  interface Props {
    variant?: 'default' | 'outline' | 'ghost' | 'destructive' | 'secondary' | 'link';
    size?: 'sm' | 'default' | 'lg';
    disabled?: boolean;
    children: import('svelte').Snippet;
    onclick?: (e: MouseEvent) => void;
  }
  let { variant = 'default', size = 'default', disabled = false, children, onclick }: Props = $props();

  const baseClasses = 'inline-flex items-center justify-center font-medium transition-colors duration-100 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--focus-ring)] disabled:pointer-events-none disabled:opacity-50';
  const variants = {
    default: 'bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)]',
    destructive: 'bg-[var(--color-error-500)] text-white hover:bg-red-600',
    outline: 'border border-[var(--border-color)] bg-transparent hover:bg-[var(--hover-overlay)] text-[var(--text-primary)]',
    secondary: 'bg-[var(--background-alt)] text-[var(--text-primary)] hover:bg-[var(--hover-overlay)]',
    ghost: 'bg-transparent hover:bg-[var(--hover-overlay)] text-[var(--text-primary)]',
    link: 'underline-offset-4 hover:underline text-[var(--accent)]'
  };
  const sizes = {
    sm: 'h-9 px-3 text-sm rounded-[var(--radius-md)]',
    default: 'h-10 px-4 text-sm rounded-[var(--radius-md)]',
    lg: 'h-11 px-8 text-sm rounded-[var(--radius-md)]'
  };
</script>
<button class="{baseClasses} {variants[variant]} {sizes[size]}" {disabled} {onclick}>
  {@render children()}
</button>
```

### Card
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { children: Snippet; class?: string; }
  let { children, class: className = '' }: Props = $props();
</script>
<div class="rounded-[var(--radius-lg)] border border-[var(--border-color)] bg-[var(--card-bg)] p-4 {className}">
  {@render children()}
</div>
```

### Data Table (dense)
```svelte
<div class="rounded-[var(--radius-lg)] border border-[var(--border-color)] overflow-hidden">
  <table class="w-full text-sm">
    <thead>
      <tr class="border-b border-[var(--border-color)] bg-[var(--background-alt)]">
        <th class="px-3 py-2 text-left text-xs font-medium uppercase tracking-wide text-[var(--text-muted)]">
          Column
        </th>
      </tr>
    </thead>
    <tbody>
      <tr class="border-b border-[var(--border-color)] hover:bg-[var(--hover-overlay)] transition-colors duration-100">
        <td class="px-3 py-2 text-[var(--text-primary)]">Cell</td>
      </tr>
    </tbody>
  </table>
</div>
```

### Input
```svelte
<script lang="ts">
  interface Props { id?: string; label: string; value?: string; placeholder?: string; error?: string; }
  let { id = `field-${Math.random().toString(36).slice(2, 9)}`, label, value = $bindable(''), placeholder = '', error }: Props = $props();
</script>
<div class="space-y-1.5">
  <label for={id} class="text-sm font-medium text-[var(--text-primary)]">{label}</label>
  <input {id} bind:value {placeholder}
    class="h-10 w-full rounded-[var(--radius-md)] border px-3 text-sm transition-colors duration-100
      {error ? 'border-[var(--color-error-500)]' : 'border-[var(--border-color)]'}
      bg-transparent text-[var(--text-primary)] placeholder:text-[var(--text-muted)]
      focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[var(--focus-ring)]"
    aria-invalid={!!error}
    aria-describedby={error ? `${id}-error` : undefined} />
  {#if error}
    <p id="{id}-error" class="text-xs text-[var(--color-error-500)]">{error}</p>
  {/if}
</div>
```

### Badge (shadcn pattern)
```svelte
<script lang="ts">
  type Variant = 'default' | 'secondary' | 'destructive' | 'outline';
  interface Props { variant?: Variant; children: import('svelte').Snippet; }
  let { variant = 'default', children }: Props = $props();
  const classes: Record<Variant, string> = {
    default: 'bg-[var(--accent)] text-white',
    secondary: 'bg-[var(--background-alt)] text-[var(--text-primary)]',
    destructive: 'bg-[var(--color-error-500)] text-white',
    outline: 'bg-transparent text-[var(--text-primary)] border-[var(--border-color)]'
  };
</script>
<span class="inline-flex items-center rounded-full border px-2 py-0.5 text-xs font-medium {classes[variant]}">
  {@render children()}
</span>
```

### Modal

shadcn/ui dialog uses `data-[state=open]:animate-in data-[state=closed]:animate-out` with `fade-in`/`fade-out` and `zoom-in-95`/`zoom-out-95`.

```svelte
<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Snippet } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  interface Props { open: boolean; title: string; children: Snippet; onclose: () => void; }
  let { open, title, children, onclose }: Props = $props();
</script>
{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="fixed inset-0 bg-black/50 backdrop-blur-sm" transition:fade={{ duration: 150 }}
      onclick={onclose} role="presentation"></div>
    <div class="relative z-10 w-full max-w-md rounded-[var(--radius-lg)] border border-[var(--border-color)]
      bg-[var(--background)] p-6 shadow-lg"
      transition:scale={{ start: 0.95, duration: 150 }}
      role="dialog" aria-modal="true" aria-label={title} tabindex="-1">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-base font-semibold text-[var(--text-primary)]">{title}</h2>
        <button onclick={onclose} class="rounded-[var(--radius-md)] p-1.5 hover:bg-[var(--hover-overlay)] transition-colors duration-100"
          aria-label="Close">
          <X class="h-4 w-4 text-[var(--text-muted)]" />
        </button>
      </div>
      {@render children()}
    </div>
  </div>
{/if}
```

### Navigation Tabs
```svelte
<script lang="ts">
  interface Props { tabs: { id: string; label: string }[]; active: string; onchange: (id: string) => void; }
  let { tabs, active, onchange }: Props = $props();
</script>
<div class="flex border-b border-[var(--border-color)]" role="tablist">
  {#each tabs as tab}
    <button role="tab" aria-selected={active === tab.id}
      class="px-3 py-2 text-sm font-medium transition-colors duration-100 -mb-px
        {active === tab.id
          ? 'border-b-2 border-[var(--accent)] text-[var(--text-primary)]'
          : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
      onclick={() => onchange(tab.id)}>
      {tab.label}
    </button>
  {/each}
</div>
```

## Animation

Apple Core Animation default: **250ms** with `cubic-bezier(0.25, 0.1, 0.25, 1.0)` (slight ease-in, strong ease-out). shadcn/ui uses `duration-200` (200ms) and `duration-150` (150ms) for most transitions.

| Type | Duration | Easing |
|------|----------|--------|
| Micro-interactions | 100-150ms | `ease-out` |
| Content transitions | 150-200ms | `cubic-bezier(0.25, 0.1, 0.25, 1.0)` |
| Modal enter | 150ms | `zoom-in-95 + fade-in` |
| Modal exit | 150ms | `zoom-out-95 + fade-out` |
| Accordion | 300ms | `ease-out` |

- **No decorative animation** — motion serves function only
- **Button feedback**: Background-color transition, no scale
- **Card hover**: Border-color change, no shadow escalation
- **Reduced motion**: All animations disabled via `prefers-reduced-motion: reduce`

## Accessibility

Apple HIG mandates: 44x44pt minimum touch targets, 4.5:1 contrast for normal text, 3:1 for large text/UI.

- **Focus ring**: `outline: 2px solid var(--focus-ring); outline-offset: 2px` (shadcn: ring-2 ring-ring ring-offset-2)
- **Touch targets**: 44px for primary actions (h-11), 36px minimum for secondary (h-9)
- **Disabled**: `opacity-50` + `pointer-events-none` (shadcn universal pattern)
- **Contrast**: 4.5:1 minimum text, 3:1 UI components (WCAG AA + Apple HIG aligned)
- **Screen reader**: `.sr-only` utility class
- **High contrast**: `@media (forced-colors: active)` support

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]`, `bg-[var(--card-bg)]`, `bg-[var(--background-alt)]`
- **Text**: `text-[var(--text-primary)]`, `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: `border-[var(--border-color)]` — borders over shadows always
- **Hover**: `hover:bg-[var(--hover-overlay)]` — subtle, no color shift
- **Sizes**: `h-9` (36px sm), `h-10` (40px default), `h-11` (44px lg)
- **Icon sizes**: `h-3.5 w-3.5` (inline), `h-4 w-4` (standard), `h-5 w-5` (nav)

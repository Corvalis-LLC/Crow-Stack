# Monolith Design Set

The Obelisk meets covert ops gothic. Matte black slabs layered with subtle depth. Cold white and pale amber accents — light hitting obsidian. Sharp geometric edges, zero border-radius. Sentinel sigils, dossier-formatted debates, threat-colored alerts. A war room for something you shouldn't be running.

**Inspirations**: 2001: A Space Odyssey monolith, Bloomberg Terminal, EVE Online UI, covert intelligence dashboards

## Color Tokens

Dark-only design system. There is no light mode. The Monolith does not answer to the sun.

### Dark Mode (Only Mode)
```css
/* Surface layers — matte black with subtle depth separation */
--background: #0c0c0e;            /* deepest void — page background */
--background-alt: #131316;         /* raised layer — cards, panels */
--background-elevated: #1a1a1f;    /* highest layer — modals, dropdowns, active states */
--background-subtle: #101013;      /* between void and alt — section backgrounds */

/* Text — cold white hierarchy through opacity */
--text-primary: #e8e8ec;           /* high-contrast body text — not pure white (too harsh) */
--text-secondary: #8a8a96;         /* supporting text, labels */
--text-muted: #55555f;             /* disabled, tertiary, metadata */
--text-accent: #d4cfc4;            /* pale amber — warm accent text for emphasis */

/* Borders — barely visible structure lines */
--border-color: rgba(255, 255, 255, 0.08);       /* default border — whisper of light */
--border-color-light: rgba(255, 255, 255, 0.04);  /* subtle dividers */
--border-color-strong: rgba(255, 255, 255, 0.14); /* emphasized borders — active cards */

/* Accent — cold white and pale amber. Light on obsidian. */
--accent: #d4cfc4;                 /* pale amber — primary accent (buttons, links, highlights) */
--accent-hover: #e8e4d8;           /* amber brightens on hover */
--accent-active: #bfb9a8;          /* amber dims on press */
--accent-muted: rgba(212, 207, 196, 0.15); /* amber glow — background tint */

/* Interaction overlays */
--hover-overlay: rgba(255, 255, 255, 0.03);  /* ghost hover — barely perceptible */
--active-overlay: rgba(255, 255, 255, 0.06); /* active press */
--focus-ring: #d4cfc4;                        /* pale amber focus ring */

/* Cards and panels */
--card-bg: #131316;
--card-border: rgba(255, 255, 255, 0.08);
--card-shadow: none;                           /* no shadows — borders define structure */
--card-shadow-hover: none;

/* Threat-level semantic colors — the war room palette */
--color-critical: #cc0000;         /* blood red — CRITICAL alerts, unanimous consensus */
--color-critical-bg: rgba(204, 0, 0, 0.08);
--color-critical-border: rgba(204, 0, 0, 0.25);
--color-high: #c49a2a;             /* sickly amber — HIGH alerts, 2-of-3 consensus */
--color-high-bg: rgba(196, 154, 42, 0.08);
--color-high-border: rgba(196, 154, 42, 0.25);
--color-neutral: #5a5a66;          /* cold steel — no consensus, neutral states */
--color-neutral-bg: rgba(90, 90, 102, 0.08);
--color-neutral-border: rgba(90, 90, 102, 0.25);

/* Directional colors — bullish/bearish */
--color-bullish: #2a8a4a;          /* dark green — not neon, restrained */
--color-bearish: #a63030;          /* dark red — not neon, restrained */

/* Standard semantic (for non-alert UI elements) */
--color-success: #2a8a4a;
--color-warning: #c49a2a;
--color-error: #cc0000;

/* ARBITER — distinct from the Triumvirate */
--color-arbiter: #6a5acd;          /* slate blue — Claude's voice, distinct from amber/red */
--color-arbiter-bg: rgba(106, 90, 205, 0.08);
--color-arbiter-border: rgba(106, 90, 205, 0.25);
```

### Sentinel Sigil Colors

Each sentinel has a signature color for its sigil and status indicators:

```css
--sentinel-argus: #4a7a8a;     /* cold teal — on-chain, blockchain */
--sentinel-echo: #7a6a9a;      /* muted purple — Western social */
--sentinel-tengu: #8a4a4a;     /* dark crimson — Asian markets, nocturnal */
--sentinel-chronos: #8a8a5a;   /* aged gold — price action, time */
--sentinel-leviathan: #3a6a7a; /* deep ocean — whale movements */
--sentinel-hermes: #6a7a5a;    /* olive — news, information */
```

## Typography

Two font stacks: monospace for data (the default), thin sans-serif for hierarchy.

```css
/* Primary: monospace — this is a data terminal */
font-family: 'JetBrains Mono', 'SF Mono', 'Cascadia Code', 'Fira Code', monospace;

/* Display: ultra-thin sans for headings and the logo */
font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
```

| Role | Size | Weight | Font | Notes |
|------|------|--------|------|-------|
| Monolith Logo | 1.5rem (24px) | 200 | Display | Ultra-thin, all caps, letter-spacing 0.3em |
| Page Title | 1.25rem (20px) | 300 | Display | Light weight, understated authority |
| Section Head | 1rem (16px) | 400 | Display | Normal weight, uppercase, tracking 0.08em |
| Headline | 0.875rem (14px) | 500 | Display | Medium weight for subsections |
| Body | 0.8125rem (13px) | 400 | Mono | Default reading text — slightly smaller for density |
| Data | 0.8125rem (13px) | 400 | Mono | Numbers, values, metrics — tabular-nums always |
| Label | 0.75rem (12px) | 500 | Display | Uppercase, tracking 0.05em, muted color |
| Caption | 0.6875rem (11px) | 400 | Mono | Timestamps, metadata |
| Minimum | 0.625rem (10px) | 400 | Mono | Micro data, chart labels |

- **Tabular numerals** everywhere — all numbers use `font-[tabular-nums]`
- **Letter spacing**: 0.3em on MONOLITH logo, 0.05-0.08em on uppercase labels, 0 elsewhere
- **No bold headings** — authority through spacing and size, not weight
- **All-caps sparingly** — logo, section labels, alert levels only

## Spacing

8pt grid, biased toward density. This is a data terminal, not a marketing site.

- **Grid**: 4, 8, 12, 16, 24, 32 — tight increments
- **Card padding**: 12px (0.75rem) — compact
- **Button padding**: 6px 12px (dense)
- **Input padding**: 6px 8px
- **Table cell padding**: 4px 8px (very dense)
- **Gap scale**: gap-1 (4px), gap-2 (8px), gap-3 (12px), gap-4 (16px)
- **Section gap**: 24px between major sections
- **Whitespace**: 20-35% — dense, every pixel is data

## Elevation

**No shadows. Ever.** Structure is defined by borders and background layer differences. The Monolith is flat — slabs of obsidian stacked at different depths.

```css
--card-shadow: none;
--card-shadow-hover: none;
```

- Depth is communicated through background color steps: `--background` → `--background-alt` → `--background-elevated`
- Active/focused elements use `--border-color-strong` instead of shadow
- Modals use a dark backdrop (`rgba(0,0,0,0.7)`) with no shadow on the panel

## Border Radius

**Zero. The Monolith has no curves.**

```css
--radius-sm: 0;
--radius-md: 0;
--radius-lg: 0;
--radius: 0;
```

Every element is a sharp rectangle. Buttons, cards, inputs, badges, modals — all square-cornered. This is non-negotiable. The harshness is the aesthetic.

**Exception**: Sentinel sigils may use `border-radius: 50%` if rendered as circular glyphs.

## Component Cheat Sheet

### Button — Sharp, Minimal
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'default' | 'accent' | 'ghost' | 'critical' | 'outline';
    size?: 'sm' | 'default' | 'lg';
    disabled?: boolean;
    children: Snippet;
    onclick?: (e: MouseEvent) => void;
  }
  let { variant = 'default', size = 'default', disabled = false, children, onclick }: Props = $props();

  const variants = {
    default: 'bg-[var(--background-elevated)] text-[var(--text-primary)] border border-[var(--border-color)] hover:border-[var(--border-color-strong)] hover:bg-[var(--hover-overlay)]',
    accent: 'bg-[var(--accent-muted)] text-[var(--accent)] border border-[var(--accent)] hover:bg-[var(--accent)] hover:text-[var(--background)]',
    ghost: 'bg-transparent text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--hover-overlay)]',
    critical: 'bg-[var(--color-critical-bg)] text-[var(--color-critical)] border border-[var(--color-critical-border)] hover:bg-[var(--color-critical)] hover:text-white',
    outline: 'bg-transparent text-[var(--text-primary)] border border-[var(--border-color)] hover:border-[var(--border-color-strong)]'
  };
  const sizes = {
    sm: 'h-7 px-2 text-xs',
    default: 'h-8 px-3 text-xs font-medium',
    lg: 'h-9 px-4 text-sm font-medium'
  };
</script>
<button class="inline-flex items-center justify-center font-mono transition-colors duration-75
  focus-visible:outline-1 focus-visible:outline-offset-1 focus-visible:outline-[var(--focus-ring)]
  disabled:pointer-events-none disabled:opacity-30
  {variants[variant]} {sizes[size]}" {disabled} {onclick}>
  {@render children()}
</button>
```

### Card — Obsidian Slab
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { title?: string; children: Snippet; class?: string; }
  let { title, children, class: className = '' }: Props = $props();
</script>
<div class="border border-[var(--border-color)] bg-[var(--background-alt)] {className}">
  {#if title}
    <div class="px-3 py-2 border-b border-[var(--border-color)]">
      <h3 class="text-xs font-medium uppercase tracking-widest text-[var(--text-muted)]">{title}</h3>
    </div>
  {/if}
  <div class="p-3">
    {@render children()}
  </div>
</div>
```

### Alert Card — Threat Level
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    level: 'critical' | 'high' | 'neutral';
    children: Snippet;
  }
  let { level, children }: Props = $props();

  const levelStyles = {
    critical: 'border-[var(--color-critical-border)] bg-[var(--color-critical-bg)]',
    high: 'border-[var(--color-high-border)] bg-[var(--color-high-bg)]',
    neutral: 'border-[var(--color-neutral-border)] bg-[var(--color-neutral-bg)]'
  };
  const levelLabels = {
    critical: { text: 'CRITICAL', color: 'text-[var(--color-critical)]' },
    high: { text: 'HIGH', color: 'text-[var(--color-high)]' },
    neutral: { text: 'NEUTRAL', color: 'text-[var(--color-neutral)]' }
  };
</script>
<div class="border {levelStyles[level]}">
  <div class="px-3 py-2 border-b border-inherit flex items-center justify-between">
    <span class="text-[10px] font-mono font-bold uppercase tracking-[0.15em] {levelLabels[level].color}">
      {levelLabels[level].text}
    </span>
    <span class="text-[10px] font-mono text-[var(--text-muted)]">
      <!-- timestamp -->
    </span>
  </div>
  <div class="p-3">
    {@render children()}
  </div>
</div>
```

### Sentinel Status Card — Sigil + Status
```svelte
<script lang="ts">
  interface Props {
    name: string;
    status: 'active' | 'idle' | 'error';
    lastPoll?: string;
    urgency?: number;
  }
  let { name, status, lastPoll, urgency = 0 }: Props = $props();

  const statusIndicator = {
    active: 'bg-[var(--color-success)]',
    idle: 'bg-[var(--text-muted)]',
    error: 'bg-[var(--color-error)]'
  };
</script>
<div class="border border-[var(--border-color)] bg-[var(--background-alt)] p-3">
  <div class="flex items-center justify-between mb-2">
    <div class="flex items-center gap-2">
      <div class="w-2 h-2 {statusIndicator[status]}"></div>
      <span class="text-xs font-mono font-bold uppercase tracking-[0.1em] text-[var(--text-primary)]">
        {name}
      </span>
    </div>
    <span class="text-[10px] font-mono text-[var(--text-muted)]">{status}</span>
  </div>
  {#if lastPoll}
    <div class="text-[10px] font-mono text-[var(--text-muted)]">Last: {lastPoll}</div>
  {/if}
  {#if urgency > 0}
    <div class="mt-1 text-xs font-mono text-[var(--color-high)]">URGENCY {urgency}/3</div>
  {/if}
</div>
```

### Debate Dossier — Intercepted Transmission Format
```svelte
<script lang="ts">
  interface JudgeVote {
    name: string;
    model: string;
    rating: string;
    confidence: number;
    reasoning: string;
  }
  interface Props {
    judges: JudgeVote[];
    consensus: string;
    dissenter?: string;
  }
  let { judges, consensus, dissenter }: Props = $props();
</script>
<div class="border border-[var(--border-color)] bg-[var(--background-alt)]">
  <div class="px-3 py-2 border-b border-[var(--border-color)] flex items-center justify-between">
    <span class="text-[10px] font-mono uppercase tracking-[0.15em] text-[var(--text-muted)]">
      Triumvirate Debate
    </span>
    <span class="text-[10px] font-mono uppercase tracking-[0.15em] text-[var(--text-accent)]">
      {consensus}
    </span>
  </div>
  {#each judges as judge}
    <div class="px-3 py-3 border-b border-[var(--border-color-light)]">
      <div class="flex items-center justify-between mb-1">
        <span class="text-xs font-mono font-bold text-[var(--text-primary)]">{judge.name}</span>
        <div class="flex items-center gap-2">
          <span class="text-[10px] font-mono text-[var(--text-muted)]">{judge.model}</span>
          <span class="text-xs font-mono font-bold
            {judge.rating.includes('BUY') ? 'text-[var(--color-bullish)]' : judge.rating.includes('AVOID') ? 'text-[var(--color-bearish)]' : 'text-[var(--text-secondary)]'}">
            {judge.rating}
          </span>
        </div>
      </div>
      <p class="text-xs font-mono text-[var(--text-secondary)] leading-relaxed">{judge.reasoning}</p>
    </div>
  {/each}
  {#if dissenter}
    <div class="px-3 py-2 bg-[var(--background-subtle)]">
      <span class="text-[10px] font-mono text-[var(--text-muted)]">DISSENT: {dissenter}</span>
    </div>
  {/if}
</div>
```

### ARBITER Report Panel
```svelte
<script lang="ts">
  interface Props {
    directionalCall: string;
    confidence: number;
    synthesis: string;
    blindSpots: string[];
    riskFactors: string[];
  }
  let { directionalCall, confidence, synthesis, blindSpots, riskFactors }: Props = $props();
</script>
<div class="border border-[var(--color-arbiter-border)] bg-[var(--color-arbiter-bg)]">
  <div class="px-3 py-2 border-b border-[var(--color-arbiter-border)] flex items-center justify-between">
    <span class="text-[10px] font-mono uppercase tracking-[0.15em] text-[var(--color-arbiter)]">
      ARBITER Synthesis
    </span>
    <div class="flex items-center gap-3">
      <span class="text-xs font-mono font-bold text-[var(--color-arbiter)]">{directionalCall}</span>
      <span class="text-[10px] font-mono text-[var(--text-muted)]">CONF {confidence}/100</span>
    </div>
  </div>
  <div class="p-3 space-y-3">
    <p class="text-xs font-mono text-[var(--text-secondary)] leading-relaxed">{synthesis}</p>
    {#if blindSpots.length > 0}
      <div>
        <span class="text-[10px] font-mono uppercase tracking-[0.1em] text-[var(--color-warning)]">Blind Spots</span>
        {#each blindSpots as spot}
          <p class="text-xs font-mono text-[var(--text-secondary)] mt-1">- {spot}</p>
        {/each}
      </div>
    {/if}
    {#if riskFactors.length > 0}
      <div>
        <span class="text-[10px] font-mono uppercase tracking-[0.1em] text-[var(--color-critical)]">Risk Factors</span>
        {#each riskFactors as risk}
          <p class="text-xs font-mono text-[var(--text-secondary)] mt-1">- {risk}</p>
        {/each}
      </div>
    {/if}
  </div>
</div>
```

### Data Table — Dense Terminal
```svelte
<div class="border border-[var(--border-color)] overflow-hidden">
  <div class="overflow-x-auto">
    <table class="w-full text-xs font-mono">
      <thead>
        <tr class="border-b border-[var(--border-color)] bg-[var(--background-subtle)]">
          <th class="px-2 py-1.5 text-left text-[10px] font-medium uppercase tracking-wider text-[var(--text-muted)]">
            Column
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-[var(--border-color-light)]">
        <tr class="hover:bg-[var(--hover-overlay)] transition-colors duration-75">
          <td class="px-2 py-1.5 text-[var(--text-primary)] whitespace-nowrap font-[tabular-nums]">
            Cell
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
```

### Input — Sharp, Minimal
```svelte
<script lang="ts">
  interface Props { id?: string; label?: string; value?: string; placeholder?: string; error?: string; }
  let { id = `field-${Math.random().toString(36).slice(2, 9)}`, label, value = $bindable(''), placeholder = '', error }: Props = $props();
</script>
<div class="space-y-1">
  {#if label}
    <label for={id} class="text-[10px] font-mono uppercase tracking-wider text-[var(--text-muted)]">{label}</label>
  {/if}
  <input {id} bind:value {placeholder}
    class="h-8 w-full border px-2 text-xs font-mono transition-colors duration-75
      {error ? 'border-[var(--color-error)]' : 'border-[var(--border-color)]'}
      bg-[var(--background)] text-[var(--text-primary)] placeholder:text-[var(--text-muted)]
      focus-visible:outline-1 focus-visible:outline-offset-0 focus-visible:outline-[var(--focus-ring)]
      focus-visible:border-[var(--accent)]"
    aria-invalid={!!error}
    aria-describedby={error ? `${id}-error` : undefined} />
  {#if error}
    <p id="{id}-error" class="text-[10px] font-mono text-[var(--color-error)]">{error}</p>
  {/if}
</div>
```

### Badge — Threat Level Indicator
```svelte
<script lang="ts">
  type Variant = 'critical' | 'high' | 'neutral' | 'bullish' | 'bearish' | 'arbiter';
  interface Props { variant?: Variant; children: import('svelte').Snippet; }
  let { variant = 'neutral', children }: Props = $props();
  const classes: Record<Variant, string> = {
    critical: 'bg-[var(--color-critical-bg)] text-[var(--color-critical)] border-[var(--color-critical-border)]',
    high: 'bg-[var(--color-high-bg)] text-[var(--color-high)] border-[var(--color-high-border)]',
    neutral: 'bg-[var(--color-neutral-bg)] text-[var(--color-neutral)] border-[var(--color-neutral-border)]',
    bullish: 'bg-transparent text-[var(--color-bullish)] border-[var(--color-bullish)]',
    bearish: 'bg-transparent text-[var(--color-bearish)] border-[var(--color-bearish)]',
    arbiter: 'bg-[var(--color-arbiter-bg)] text-[var(--color-arbiter)] border-[var(--color-arbiter-border)]'
  };
</script>
<span class="inline-flex items-center border px-1.5 py-0.5 text-[10px] font-mono font-bold uppercase tracking-wider {classes[variant]}">
  {@render children()}
</span>
```

## Animation

Minimal. The Monolith is still — it doesn't bounce, slide, or wiggle. Motion is reserved for state changes and data flow.

| Type | Duration | Easing | Notes |
|------|----------|--------|-------|
| Hover states | 75ms | `ease-out` | Background/border color only |
| Alert arrival | 200ms | `ease-out` | Fade in + subtle opacity shift |
| Alert pulse | 2000ms | `ease-in-out` | Slow, breathing glow on CRITICAL alerts — `animate-pulse` with low opacity |
| Data update | 150ms | `ease-out` | Number/text content change |
| Modal | 150ms | `ease-out` | Fade backdrop, no scale/slide |
| Page transition | 0ms | None | Instant. No page transitions. |

- **No decorative animation** — if it moves, it means something
- **Alert breathing**: CRITICAL alerts have a slow, barely-perceptible pulse on their border — like something alive
- **Reduced motion**: All animation disabled via `prefers-reduced-motion: reduce`
- **No hover scale, no hover lift, no hover shadow** — color changes only

## Accessibility

Dark-only design must still meet contrast requirements:

- **Focus ring**: `outline: 1px solid var(--focus-ring); outline-offset: 1px` — thin, sharp, visible
- **Touch targets**: 32px minimum (h-8), 28px for compact controls (h-7)
- **Contrast**: `--text-primary` (#e8e8ec) on `--background` (#0c0c0e) = 16.3:1 (exceeds AAA)
- **Contrast**: `--text-secondary` (#8a8a96) on `--background` (#0c0c0e) = 6.1:1 (exceeds AA)
- **Contrast**: `--text-muted` (#55555f) on `--background` (#0c0c0e) = 3.3:1 (meets AA for large text/UI)
- **Alert colors**: All threat colors meet 3:1 against their background tints
- **Screen reader**: `.sr-only` utility, proper ARIA labels on all interactive elements
- **Keyboard**: Full keyboard navigation, visible focus indicators
- **High contrast**: `@media (forced-colors: active)` support for Windows High Contrast

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]`, `bg-[var(--background-alt)]`, `bg-[var(--background-elevated)]`
- **Text**: `text-[var(--text-primary)]`, `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: `border-[var(--border-color)]` — borders define all structure, no shadows
- **Hover**: `hover:bg-[var(--hover-overlay)]` or `hover:border-[var(--border-color-strong)]`
- **Sizes**: `h-7` (compact), `h-8` (default), `h-9` (large)
- **Font**: `font-mono` is the default. `font-sans` only for display headings.
- **Radius**: Never applied. All elements are sharp rectangles.
- **Icon sizes**: `h-3 w-3` (micro), `h-3.5 w-3.5` (inline), `h-4 w-4` (standard)
- **Numbers**: Always `font-[tabular-nums]`
- **Labels**: `text-[10px] uppercase tracking-wider font-mono text-[var(--text-muted)]`

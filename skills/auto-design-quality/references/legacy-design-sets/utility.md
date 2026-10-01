# Utility Design Set

GitHub Primer inspired. Dense, muted, functional. Maximum information density for admin panels, internal tools, developer dashboards. Every pixel earns its place.

**Sources**: [Primer Style](https://primer.style/), [Primer Primitives](https://github.com/primer/primitives), [Primer Typography](https://primer.style/foundations/typography/), [Primer CSS Spacing](https://primer.style/css/support/spacing/), [Primer Box Shadow](https://primer.style/foundations/css-utilities/box-shadow/), [Primer Borders](https://primer.style/product/css-utilities/borders/), [Primer Color System (GitHub Blog)](https://github.blog/engineering/user-experience/unlocking-inclusive-design-how-primers-color-system-is-making-github-com-more-inclusive/)

## Color Tokens

### Light Mode

From [Primer Primitives](https://primer.style/primitives/colors/) — functional color system with blue-tinted grays:

```css
/* Primer scale.gray — light theme (from primer/primitives) */
--background: #ffffff;           /* scale.white */
--background-subtle: #f6f8fa;    /* scale.gray.0 — subtle backgrounds */
--background-elevated: #ffffff;
--text-primary: #1f2328;         /* fg.default — near-black with blue tint */
--text-secondary: #656d76;       /* fg.muted */
--text-muted: #6e7781;           /* scale.gray.5 */
--border-color: #d0d7de;         /* scale.gray.2 — borders */
--border-color-light: #eaeef2;   /* scale.gray.1 — subtle borders */
--accent: #0366D6;               /* Primer blue — documented link/accent color */
--accent-hover: #0550AE;
--accent-active: #033d8b;
--hover-overlay: rgba(208, 215, 222, 0.32);  /* Primer hover on neutral */
--active-overlay: rgba(208, 215, 222, 0.48);
--focus-ring: #0366D6;
--card-bg: var(--background);
--card-border: var(--border-color);
```

### Dark Mode

From [Primer Primitives](https://primer.style/primitives/colors/) — dark theme:

```css
/* Primer dark theme — from primer/primitives */
--background: #0d1117;          /* dark canvas default */
--background-subtle: #161b22;   /* dark canvas subtle */
--background-elevated: #21262d; /* dark canvas elevated */
--text-primary: #e6edf3;        /* dark fg.default */
--text-secondary: #8b949e;      /* dark fg.muted — scale.gray.4 */
--text-muted: #6e7781;          /* scale.gray.5 */
--border-color: #30363d;        /* dark border.default */
--border-color-light: #484f58;  /* dark border.subtle */
--accent: #58a6ff;              /* dark Primer blue accent */
--accent-hover: #79c0ff;
--accent-active: #388bfd;
--hover-overlay: rgba(177, 186, 196, 0.12);
--active-overlay: rgba(177, 186, 196, 0.2);
--focus-ring: #58a6ff;
--card-bg: var(--background);
--card-border: var(--border-color);
```

### Full Gray Scale

From [Primer Primitives](https://github.com/primer/primitives) — `scale.gray` (blue-tinted neutrals):

```css
/* scale.white  */ #FFFFFF
/* scale.gray.0 */ #F6F8FA    /* subtle bg */
/* scale.gray.1 */ #EAEEF2    /* subtle border */
/* scale.gray.2 */ #D0D7DE    /* border */
/* scale.gray.3 */ #AFB8C1    /* muted icon */
/* scale.gray.4 */ #8C959F    /* placeholder */
/* scale.gray.5 */ #6E7781    /* secondary text */
/* scale.gray.6 */ #57606A    /* body text alt */
/* scale.gray.7 */ #424A53    /* strong secondary */
/* scale.gray.8 */ #32383F    /* emphasis */
/* scale.gray.9 */ #24292F    /* heading text */
/* scale.black  */ #1B1F24    /* near-black */
```

### Semantic Colors

From [Primer Color System](https://github.blog/engineering/user-experience/unlocking-inclusive-design-how-primers-color-system-is-making-github-com-more-inclusive/):

```css
/* light / dark */
--color-success: #1a7f37 / #3fb950;  /* green */
--color-warning: #9a6700 / #d29922;  /* yellow */
--color-error: #cf222e / #f85149;    /* red */
--color-accent: #0366D6 / #58a6ff;   /* blue */
```

## Typography

### Font Stacks

From [Primer Typography](https://primer.style/foundations/typography/) — system font stack (no custom web fonts):

```css
/* Primer system font stack — documented at primer.style/foundations/typography */
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans",
  Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";

/* Monospace stack */
font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
```

### Type Scale

From [Primer CSS Typography Utilities](https://primer.style/foundations/css-utilities/typography/) — `.f00` through `.f6`:

| Class | Size (px) | Rem | Role | Weight |
|-------|-----------|-----|------|--------|
| `.f00` | 40px | 2.5rem | Marketing hero | Light (300) available |
| `.f0` | 32px | 2rem | Page title | Light (300) available |
| `.f1` | 26px | 1.625rem | Section heading | Light (300) available |
| `.f2` | 22px | 1.375rem | Subsection | Light (300) available |
| `.f3` | 18px | 1.125rem | Large body | Light (300) available |
| `.f4` | 16px | 1rem | Body default | 400 normal |
| `.f5` | 14px | 0.875rem | Small body | 400 normal |
| `.f6` | 12px | 0.75rem | Caption/meta | 400 normal |

- **Default base**: 14px (`.f5`) for dense interfaces, 16px (`.f4`) for readability
- **Line heights**: Unitless values, designed to align to 4px grid
- **All sizes in rem** for accessible browser zoom

## Spacing

From [Primer CSS Spacing](https://primer.style/css/support/spacing/) — base-8 scale:

| Token | Value | Rem | Use |
|-------|-------|-----|-----|
| `$spacer-0` | 0 | 0 | Reset |
| `$spacer-1` | 4px | 0.25rem | Tight inline |
| `$spacer-2` | 8px | 0.5rem | Icon gap, tight padding |
| `$spacer-3` | 16px | 1rem | Default padding |
| `$spacer-4` | 24px | 1.5rem | Section gap |
| `$spacer-5` | 32px | 2rem | Large section gap |
| `$spacer-6` | 40px | 2.5rem | Page section spacing |

Extended scale continues through `$spacer-12` for larger layout spacing. 4px increments for small dimensions, 16px increments for larger.

- **Card padding**: 12px or 16px ($spacer-2 / $spacer-3)
- **Button padding**: 6px 12px ($spacer-1.5 / $spacer-2 equiv)
- **Table cell padding**: 6px 8px (compact, dense)
- **Whitespace**: 20-40% — dense, every pixel counts

## Elevation

From [Primer Box Shadow Utilities](https://primer.style/foundations/css-utilities/box-shadow/) and [Primer Theme Reference](https://primer.style/product/getting-started/react/theme-reference/):

```css
/* Primer shadow tokens — light mode, documented in Theme Reference */
--shadow-resting-small: 0 1px 0 rgba(31, 35, 40, 0.1);
--shadow-floating-small: 0 1px 3px rgba(31, 35, 40, 0.12), 0 8px 24px rgba(66, 74, 83, 0.12);
--shadow-floating-xlarge: 0 12px 28px rgba(140, 149, 159, 0.3);
--shadow-highlight: inset 0 1px 0 rgba(255, 255, 255, 0.25);
--shadow-inset: inset 0 1px 0 rgba(208, 215, 222, 0.2);

/* Canonical shadow mappings */
--card-shadow: var(--shadow-resting-small);
--card-shadow-hover: var(--shadow-floating-small);
```

Usage levels documented in Primer:
- **Resting small**: Pricing cards, important info containers
- **Floating small**: Dropdowns, tooltips, popovers
- **Floating xlarge**: Modals, major overlays
- **Highlight**: Inner highlight on buttons
- **Inset**: Inset shadow for inputs

Default: **Flat (no shadow)** — borders define structure. Shadows only for overlays.

## Border Radius

From [Primer Borders Utilities](https://primer.style/product/css-utilities/borders/):

```css
/* Primer border radius utilities (canonical names) */
--radius-sm: 4px;     /* buttons, inputs */
--radius-md: 6px;     /* cards, panels */
--radius-lg: 8px;     /* large containers */
/* Minimal rounding — functional, not decorative */
```

## Component Cheat Sheet

### Button (compact, Primer-style — 32px default height)

From Primer: medium button = 32px visual height (notably compact).

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'default' | 'primary' | 'outline' | 'ghost' | 'danger';
    size?: 'sm' | 'default';
    children: Snippet;
    onclick?: (e: MouseEvent) => void;
    disabled?: boolean;
  }
  let { variant = 'default', size = 'default', children, onclick, disabled = false }: Props = $props();

  const variants = {
    default: 'bg-[var(--background-subtle)] text-[var(--text-primary)] border border-[var(--border-color)] shadow-[var(--shadow-highlight)] hover:bg-[var(--hover-overlay)]',
    primary: 'bg-[var(--accent)] text-white border border-[var(--accent)] hover:bg-[var(--accent-hover)]',
    outline: 'bg-transparent text-[var(--text-primary)] border border-[var(--border-color)] hover:bg-[var(--hover-overlay)]',
    ghost: 'bg-transparent text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--hover-overlay)]',
    danger: 'bg-transparent text-[var(--color-error)] border border-[var(--border-color)] hover:bg-[var(--color-error)] hover:text-white'
  };
  const sizes = {
    sm: 'h-7 px-2 text-xs rounded-[var(--radius-sm)]',
    default: 'h-8 px-3 text-xs font-medium rounded-[var(--radius-sm)]'
  };
</script>
<button class="inline-flex items-center justify-center transition-colors duration-100
  focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[var(--focus-ring)]
  disabled:pointer-events-none disabled:opacity-50
  {variants[variant]} {sizes[size]}" {disabled} {onclick}>
  {@render children()}
</button>
```

### Card (flat, bordered — Primer pattern)
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { title?: string; children: Snippet; class?: string; }
  let { title, children, class: className = '' }: Props = $props();
</script>
<div class="rounded-[var(--radius-md)] border border-[var(--border-color)] bg-[var(--background)] {className}">
  {#if title}
    <div class="px-3 py-2 border-b border-[var(--border-color)] bg-[var(--background-subtle)]">
      <h3 class="text-xs font-semibold text-[var(--text-primary)]">{title}</h3>
    </div>
  {/if}
  <div class="p-3">
    {@render children()}
  </div>
</div>
```

### Data Table (compact, dense — Primer density)
```svelte
<div class="rounded-[var(--radius-md)] border border-[var(--border-color)] overflow-hidden">
  <div class="overflow-x-auto">
    <table class="w-full text-xs">
      <thead>
        <tr class="border-b border-[var(--border-color)] bg-[var(--background-subtle)]">
          <th class="px-2 py-1.5 text-left font-medium text-[var(--text-secondary)] whitespace-nowrap">
            Column
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-[var(--border-color)]">
        <tr class="hover:bg-[var(--hover-overlay)] transition-colors duration-75">
          <td class="px-2 py-1.5 text-[var(--text-primary)] whitespace-nowrap">Cell</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="flex items-center justify-between border-t border-[var(--border-color)] px-2 py-1.5 bg-[var(--background-subtle)]">
    <span class="text-xs text-[var(--text-muted)]">Showing 1-25 of 100</span>
    <!-- Pagination -->
  </div>
</div>
```

### Input (compact)
```svelte
<script lang="ts">
  interface Props { id?: string; label?: string; value?: string; placeholder?: string; error?: string; }
  let { id = `field-${Math.random().toString(36).slice(2, 9)}`, label, value = $bindable(''), placeholder = '', error }: Props = $props();
</script>
<div class="space-y-1">
  {#if label}
    <label for={id} class="text-xs font-medium text-[var(--text-primary)]">{label}</label>
  {/if}
  <input {id} bind:value {placeholder}
    class="h-8 w-full rounded-[var(--radius-sm)] border px-2 text-sm transition-colors duration-75
      {error ? 'border-[var(--color-error)]' : 'border-[var(--border-color)]'}
      bg-[var(--background)] text-[var(--text-primary)] placeholder:text-[var(--text-muted)]
      shadow-[var(--shadow-inset)]
      focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[var(--focus-ring)]"
    aria-invalid={!!error}
    aria-describedby={error ? `${id}-error` : undefined} />
  {#if error}
    <p id="{id}-error" class="text-xs text-[var(--color-error)]">{error}</p>
  {/if}
</div>
```

### Command Palette (keyboard-first)
```svelte
<script lang="ts">
  import { Search } from 'lucide-svelte';
  import { fade, scale } from 'svelte/transition';
  interface Props { open: boolean; onclose: () => void; }
  let { open, onclose }: Props = $props();
  let query = $state('');
</script>
{#if open}
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-[15vh]">
    <div class="fixed inset-0 bg-black/50" transition:fade={{ duration: 100 }}
      onclick={onclose} role="presentation"></div>
    <div class="relative z-10 w-full max-w-lg rounded-[var(--radius-md)] border border-[var(--border-color)]
      bg-[var(--background)] shadow-[var(--shadow-floating-xlarge)] overflow-hidden"
      transition:scale={{ start: 0.95, duration: 100 }}
      role="dialog" aria-modal="true" aria-label="Command palette" tabindex="-1">
      <div class="flex items-center gap-2 border-b border-[var(--border-color)] px-3">
        <Search class="h-4 w-4 text-[var(--text-muted)] shrink-0" />
        <input bind:value={query} placeholder="Type a command or search..."
          class="h-10 w-full bg-transparent text-sm text-[var(--text-primary)] placeholder:text-[var(--text-muted)] outline-none" />
        <kbd class="hidden sm:inline-flex items-center rounded-[var(--radius-sm)] border border-[var(--border-color)] bg-[var(--background-subtle)] px-1.5 py-0.5 text-[10px] text-[var(--text-muted)]">
          Esc
        </kbd>
      </div>
      <div class="max-h-80 overflow-y-auto p-1">
        <!-- Command items with keyboard navigation -->
      </div>
    </div>
  </div>
{/if}
```

### Code Block
```svelte
<script lang="ts">
  import { Copy, Check } from 'lucide-svelte';
  interface Props { code: string; language?: string; }
  let { code, language = '' }: Props = $props();
  let copied = $state(false);

  function copyCode() {
    navigator.clipboard.writeText(code);
    copied = true;
    setTimeout(() => { copied = false; }, 2000);
  }
</script>
<div class="rounded-[var(--radius-md)] border border-[var(--border-color)] overflow-hidden">
  <div class="flex items-center justify-between px-3 py-1.5 border-b border-[var(--border-color)] bg-[var(--background-subtle)]">
    <span class="text-xs text-[var(--text-muted)]">{language}</span>
    <button onclick={copyCode} class="rounded-[var(--radius-sm)] p-1 hover:bg-[var(--hover-overlay)] transition-colors duration-75"
      aria-label={copied ? 'Copied' : 'Copy code'}>
      {#if copied}
        <Check class="h-3.5 w-3.5 text-[var(--color-success)]" />
      {:else}
        <Copy class="h-3.5 w-3.5 text-[var(--text-muted)]" />
      {/if}
    </button>
  </div>
  <pre class="p-3 text-xs leading-relaxed font-mono text-[var(--text-primary)] bg-[var(--background-subtle)] overflow-x-auto"><code>{code}</code></pre>
</div>
```

### Tree View
```svelte
<script lang="ts">
  import { ChevronRight, File, Folder } from 'lucide-svelte';
  interface TreeNode { name: string; type: 'file' | 'folder'; children?: TreeNode[]; }
  interface Props { nodes: TreeNode[]; depth?: number; }
  let { nodes, depth = 0 }: Props = $props();
</script>
<ul class="text-xs" role="tree">
  {#each nodes as node}
    <li role="treeitem" aria-expanded={node.type === 'folder' ? true : undefined}>
      <button class="flex items-center gap-1 w-full px-2 py-0.5 hover:bg-[var(--hover-overlay)]
        rounded-[var(--radius-sm)] transition-colors duration-75 text-[var(--text-primary)]"
        style="padding-left: {depth * 16 + 8}px">
        {#if node.type === 'folder'}
          <ChevronRight class="h-3 w-3 text-[var(--text-muted)]" />
          <Folder class="h-3.5 w-3.5 text-[var(--accent)]" />
        {:else}
          <span class="w-3"></span>
          <File class="h-3.5 w-3.5 text-[var(--text-muted)]" />
        {/if}
        <span class="truncate">{node.name}</span>
      </button>
      {#if node.children}
        <svelte:self nodes={node.children} depth={depth + 1} />
      {/if}
    </li>
  {/each}
</ul>
```

## Animation

Primer uses minimal, fast transitions — GitHub's interface feels instant:

- **Micro-interactions**: 75-100ms ease-out — instant feel
- **Content transitions**: 100-150ms ease-out
- **No decorative animation** — functional only
- **Button feedback**: Background color change only, no scale
- **Card hover**: Background color change, no lift
- **Modal**: `shadow-[var(--shadow-floating-xlarge)]`, fast scale (100ms)
- **Max**: 150ms — anything longer feels sluggish in this density
- **Reduced motion**: All animations disabled via `prefers-reduced-motion: reduce`

## Accessibility
- **Focus ring**: `outline: 2px solid var(--focus-ring); outline-offset: 1px` (tighter offset for density)
- **Touch targets**: 28px minimum (compact), 32px for primary actions (Primer default button height)
- **Keyboard shortcuts**: Show `<kbd>` hints in UI
- **Screen reader**: `.sr-only` utility class
- **High contrast**: Primer supports dedicated high-contrast theme mode
- **Color vision**: Primer includes Protanopia & Deuteranopia and Tritanopia theme modes
- **Dense but readable**: 14px body, 1.5 line-height, sufficient contrast

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]`, `bg-[var(--background-subtle)]` — flat, no shadows by default
- **Text**: `text-[var(--text-primary)]`, `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: `border-[var(--border-color)]` — borders everywhere, they define structure
- **Shadows**: `shadow-[var(--shadow-resting-small)]` for resting, `shadow-[var(--shadow-floating-small)]` for overlays
- **Hover**: `hover:bg-[var(--hover-overlay)]` — subtle background change only
- **Sizes**: `h-7` (compact), `h-8` (default = 32px Primer standard) — small controls
- **Icon sizes**: `h-3 w-3` (tree), `h-3.5 w-3.5` (inline), `h-4 w-4` (standard)
- **Font**: `font-mono` for data, IDs, code, hashes

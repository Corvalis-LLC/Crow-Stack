# Dadson Design Set

Professional laundromat and vending management platform. Senior-friendly, WCAG 2.2 AA, light/dark mode, warm neutrals + professional blue accent.

## Color Tokens

### Root (Shared)
```css
--color-primary-50: #eff6ff;   --color-primary-100: #dbeafe;
--color-primary-200: #bfdbfe;  --color-primary-300: #93c5fd;
--color-primary-400: #60a5fa;  --color-primary-500: #2563eb;
--color-primary-600: #1d4ed8;  --color-primary-700: #1e40af;
--color-primary-800: #1e3a8a;  --color-primary-900: #1e3057;

--color-secondary-50: #f8fafc;  --color-secondary-500: #64748b;
--color-secondary-900: #0f172a;

--color-tertiary-50: #fffbeb;   --color-tertiary-500: #f59e0b;
--color-tertiary-900: #78350f;
```

### Light Mode `[data-theme='dadson']`
```css
--background: #f8f9fa;  --background-alt: #ffffff;  --background-elevated: #ffffff;
--text-primary: #1f2937;  --text-secondary: #4b5563;  --text-muted: #6b7280;
--border-color: #e5e7eb;  --border-color-light: #d1d5db;
--accent: var(--color-primary-500);  --accent-hover: var(--color-primary-600);
--accent-active: var(--color-primary-700);
--hover-overlay: rgba(37, 99, 235, 0.04);  --active-overlay: rgba(37, 99, 235, 0.08);
--focus-ring: var(--color-primary-500);
--card-bg: #ffffff;  --card-border: #e5e7eb;
--card-shadow: 0 1px 3px rgba(0,0,0,0.08), 0 1px 2px rgba(0,0,0,0.04);
--card-shadow-hover: 0 4px 6px rgba(0,0,0,0.07), 0 2px 4px rgba(0,0,0,0.05);
```

### Dark Mode `[data-theme='dadson-dark']`
```css
--background: #0c1220;  --background-alt: #111827;  --background-elevated: #1e293b;
--text-primary: #f1f5f9;  --text-secondary: #cbd5e1;  --text-muted: #94a3b8;
--border-color: #293548;  --border-color-light: #374151;
--accent: #60a5fa;  --accent-hover: #93c5fd;
--accent-active: var(--color-primary-500);
--hover-overlay: rgba(96, 165, 250, 0.06);  --active-overlay: rgba(96, 165, 250, 0.12);
--focus-ring: #60a5fa;
--card-bg: #111827;  --card-border: #293548;
--card-shadow: 0 1px 3px rgba(0,0,0,0.3), 0 1px 2px rgba(0,0,0,0.2);
--card-shadow-hover: 0 4px 6px rgba(0,0,0,0.4), 0 2px 4px rgba(0,0,0,0.3);
```

### Semantic Colors (Light / Dark inverted)
```css
/* Success: green */  --color-success-500: #059669 / #34d399;
/* Warning: amber */  --color-warning-500: #d97706 / #fbbf24;
/* Error: red */      --color-error-500: #dc2626 / #f87171;
```

## Typography
- **Font**: `'Inter', system-ui, -apple-system, sans-serif`
- **Scale**: 0.6875rem (11px labels) → 0.75rem (12px small) → 0.8125rem (13px) → 0.875rem (14px body) → 0.9375rem (15px buttons) → 1.125rem-1.75rem (headings)
- **Weights**: 400 body, 500 medium/buttons, 600 semibold/titles, 700 bold/headings
- **Line height**: 1.6 body
- **Tabular numerals** for financial data
- **16px minimum** font size for inputs (senior-friendly)
- **Table headers**: uppercase, 0.75rem, letter-spacing 0.05em, font-weight 600

## Spacing
- **Base unit**: 4px (0.25rem)
- **Grid**: 4, 8, 12, 16, 20, 24, 32, 40, 48
- **Card padding**: 1rem (16px)
- **Button padding**: 0.625rem 1.25rem (10px 20px)
- **Input padding**: 0.625rem 0.875rem (10px 14px)
- **Table cell padding**: 0.5rem 0.75rem (dense)
- **Card header**: pb-0.75rem, mb-0.75rem, border-bottom
- **Gap scale**: gap-1 (4px) through gap-8 (32px)

## Elevation
```css
--card-shadow: 0 1px 3px rgba(0,0,0,0.08), 0 1px 2px rgba(0,0,0,0.04);
--card-shadow-hover: 0 4px 6px rgba(0,0,0,0.07), 0 2px 4px rgba(0,0,0,0.05);
```
- **Dropdowns**: shadow-lg, z-50
- **Dark mode**: heavier shadows with higher opacity

## Border Radius
```css
--radius-sm: 0.25rem;  /* 4px - subtle */
--radius-md: 0.375rem; /* 6px - inputs, buttons */
--radius-lg: 0.5rem;   /* 8px - cards, panels */
--radius-xl: 0.75rem;  /* 12px - large cards */
/* rounded-full for pills, avatars */
```

## Component Cheat Sheet

### Button — Primary
```svelte
<button class="btn-primary">
  <!-- bg: var(--accent), color: white, padding: 0.625rem 1.25rem -->
  <!-- hover: var(--accent-hover), active: scale(0.98) -->
  <!-- font: 0.9375rem, weight: 500, radius: var(--radius-md) -->
  <!-- display: inline-flex, align-items: center -->
  Submit
</button>
```

### Button — Secondary
```svelte
<button class="btn-secondary">
  <!-- bg: transparent, border: 1px solid var(--border-color) -->
  <!-- hover: bg var(--hover-overlay), border var(--border-color-light) -->
  <!-- same padding, font, radius as primary -->
  Cancel
</button>
```

### Glass Card
```svelte
<div class="glass-card">
  <!-- bg: var(--card-bg), border: 1px solid var(--card-border) -->
  <!-- radius: var(--radius-lg), shadow: var(--card-shadow), padding: 1rem -->
  <!-- hover: shadow upgrades to var(--card-shadow-hover) -->
  <!-- Tables inside: padding: 0 (auto-removed by :has selector) -->

  <div class="card-header">
    <!-- flex, items-center, justify-between -->
    <!-- pb-0.75rem, mb-0.75rem, border-bottom -->
    <h3 class="card-title">{title}</h3>
    <!-- font-weight: 600, font-size: 0.9375rem -->
  </div>

  <!-- Card body content -->
</div>
```

### Status Badge (verbatim)
```svelte
<script lang="ts">
  type Variant = 'success' | 'warning' | 'error' | 'info' | 'neutral';
  interface Props { status: string; variant?: Variant; }
  let { status, variant }: Props = $props();

  const defaultVariants: Record<string, Variant> = {
    active: 'success', inactive: 'neutral', pending: 'warning',
    unassigned: 'neutral', passed: 'info', new: 'info',
    selling: 'warning', sold: 'success', dead: 'error'
  };
  const resolvedVariant = $derived(variant || defaultVariants[status.toLowerCase()] || 'neutral');
  const variantClasses: Record<Variant, string> = {
    success: 'bg-[var(--color-success-50)] text-[var(--color-success-600)]',
    warning: 'bg-[var(--color-warning-50)] text-[var(--color-warning-600)]',
    error: 'bg-[var(--color-error-50)] text-[var(--color-error-600)]',
    info: 'bg-[var(--color-primary-50)] text-[var(--color-primary-700)]',
    neutral: 'bg-[var(--color-surface-200)] text-[var(--color-surface-700)]'
  };
</script>
<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium capitalize {variantClasses[resolvedVariant]}">
  {status}
</span>
```

### Form Section (verbatim)
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props { title: string; description?: string; children: Snippet; columns?: 1 | 2 | 3 | 4; }
  let { title, description, children, columns = 2 }: Props = $props();
  const gridCols: Record<number, string> = {
    1: 'grid-cols-1',
    2: 'grid-cols-1 sm:grid-cols-2',
    3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
    4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4'
  };
</script>
<div class="glass-card">
  <div class="card-header">
    <div>
      <h3 class="card-title">{title}</h3>
      {#if description}
        <p class="mt-0.5 text-xs text-[var(--text-muted)]">{description}</p>
      {/if}
    </div>
  </div>
  <div class="grid gap-4 {gridCols[columns]}">
    {@render children()}
  </div>
</div>
```

### Data Table (pattern)
```svelte
<!-- Generic table with @vincjo/datatables -->
<div class="glass-card overflow-hidden p-0">
  <div class="overflow-x-auto">
    <table class="w-full table-dense">
      <thead>
        <tr class="border-b border-[var(--border-color)] bg-[var(--background)]">
          <th class="px-4 py-3 text-left"><!-- ThSort for sortable --></th>
        </tr>
      </thead>
      <tbody>
        <tr class="border-b border-[var(--border-color)] hover:bg-[var(--hover-overlay)]">
          <td class="px-4 py-3"><!-- cell content --></td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="flex items-center justify-between border-t border-[var(--border-color)] px-4 py-3">
    <div class="flex items-center gap-4 text-xs text-[var(--text-muted)]">
      <!-- RowCount + RowsPerPage -->
    </div>
    <!-- Pagination -->
  </div>
</div>
```

### Action Menu / Dropdown (pattern)
```svelte
<!-- Keyboard-navigable dropdown with click-outside -->
<div class="relative">
  <button class="rounded p-1 hover:bg-[var(--hover-overlay)]"
    aria-label="Actions" aria-expanded={open} aria-haspopup="true">
    <MoreVertical class="h-4 w-4" />
  </button>
  {#if open}
    <div class="absolute right-0 z-50 mt-1 min-w-[160px] overflow-hidden
      rounded-lg border border-[var(--border-color)]
      bg-[var(--background-alt)] shadow-lg"
      role="menu" tabindex="-1">
      <!-- menuitem buttons with ArrowUp/Down/Escape keyboard nav -->
      <!-- Auto-focus first item on open via $effect + tick() -->
    </div>
  {/if}
</div>
```

### Auto-Save Input (pattern)
```svelte
<!-- Debounced auto-save with status indicators -->
<!-- States: idle | saving | saved | error -->
<div class="space-y-1">
  <div class="flex items-center justify-between">
    <label class="block text-sm font-medium">
      {label} {#if required}<span class="text-[var(--color-error-500)]">*</span>{/if}
    </label>
    <div class="flex items-center gap-1 text-xs">
      <!-- saving: Loader2 spin + "Saving..." in text-muted -->
      <!-- saved: Check + "Saved" in success-500 -->
      <!-- error: AlertCircle + "Error" in error-500 -->
    </div>
  </div>
  <input class="w-full {status === 'error' ? 'border-[var(--color-error-500)]' : ''}" />
  <!-- Error message in text-xs text-[var(--color-error-500)] -->
</div>
```

### Filter Panel (verbatim)
```svelte
<script lang="ts">
  import { Filter } from 'lucide-svelte';
  import type { Snippet } from 'svelte';
  import { slide } from 'svelte/transition';
  interface Props { children: Snippet; }
  let { children }: Props = $props();
  let showFilters = $state(false);
</script>
<div>
  <button class="btn-secondary flex items-center gap-2"
    onclick={() => (showFilters = !showFilters)}
    aria-expanded={showFilters} aria-controls="filter-panel-content">
    <Filter class="h-4 w-4" /> Filters
  </button>
  {#if showFilters}
    <div id="filter-panel-content" class="mt-4" transition:slide={{ duration: 200 }}>
      <div class="glass-card space-y-3 p-4">
        {@render children()}
      </div>
    </div>
  {/if}
</div>
```

### Empty State (pattern)
```svelte
<div class="text-center py-12">
  <IconComponent class="h-12 w-12 mx-auto text-[var(--text-muted)] mb-4" />
  <h3 class="text-lg font-medium text-[var(--text-primary)]">No items yet</h3>
  <p class="text-[var(--text-muted)] mt-1">Get started by creating your first item.</p>
  <button class="btn-primary mt-4">Create Item</button>
</div>
```

### Navigation Sidebar (pattern)
```svelte
<!-- Collapsible sidebar: 64px collapsed / 264px expanded -->
<aside class="bg-[var(--background-alt)] border-r border-[var(--border-color)]">
  <!-- Nav items -->
  <a class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm
    hover:bg-[var(--hover-overlay)] text-[var(--text-secondary)]">
    <Icon class="h-5 w-5 shrink-0" />
    <span>{label}</span>
  </a>
  <!-- Active state -->
  <a class="... bg-[var(--accent)] text-white">
    <!-- ChevronRight indicator -->
  </a>
  <!-- Section labels -->
  <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)]">
    {sectionName}
  </span>
</aside>
```

## Animation
- **Micro-interactions**: 150ms ease (hovers, toggles)
- **Content transitions**: 200ms ease-out (fade-in, slide-in)
- **Layout changes**: 300ms ease (expanding content, modals)
- **Button feedback**: `transform: scale(0.98)` on `:active`
- **Card hover**: `translateY(-1px)` + shadow upgrade
- **Slide transitions**: `transition:slide={{ duration: 200 }}`
- **Max**: Never exceed 500ms
- **Reduced motion**: All animations disabled via `prefers-reduced-motion: reduce`

## Accessibility
- **Focus ring**: `outline: 2px solid var(--focus-ring); outline-offset: 2px`
- **Touch targets**: 44px minimum
- **Screen reader**: `.sr-only` utility class
- **High contrast**: `@media (forced-colors: active)` — 3px CanvasText outline
- **Scroll margin**: `scroll-margin-top: 4rem; scroll-margin-bottom: 2rem`
- **Selection**: themed colors for both light/dark
- **ARIA patterns**: `aria-expanded`, `aria-haspopup`, `aria-controls`, `role="menu"`, `role="menuitem"`
- **Keyboard nav**: ArrowUp/Down for menus, Escape to close, auto-focus first item

## Tailwind Class Patterns
- **Backgrounds**: `bg-[var(--background)]`, `bg-[var(--card-bg)]`, `bg-[var(--background-alt)]`
- **Text**: `text-[var(--text-primary)]`, `text-[var(--text-secondary)]`, `text-[var(--text-muted)]`
- **Borders**: `border-[var(--border-color)]`, `border-[var(--border-color-light)]`
- **Hover**: `hover:bg-[var(--hover-overlay)]`
- **Semantic**: `text-[var(--color-success-500)]`, `bg-[var(--color-error-50)]`
- **Icon sizes**: `h-3 w-3` (inline), `h-4 w-4` (standard), `h-5 w-5` (nav), `h-12 w-12` (empty states)

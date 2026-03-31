# Loading & Feedback States — Implementation Patterns

Detailed patterns for skeleton screens, spinners, progress bars, error states, empty states, and accessible loading. Load on demand when building data-fetching views.

---

## Timing Thresholds (Nielsen Norman Group)

- **0.1s** — Perceived as instantaneous. No feedback needed.
- **1.0s** — User's thought flow stays uninterrupted. Subtle indicator acceptable.
- **10s** — Attention limit. Must show progress with estimate.

**Key insight:** Waits with feedback feel 11–15% faster. Skeleton screens reduce perceived wait by 20–30% vs blank screens.

---

## Skeleton Screens

### When to Use

- Full page or large section loading (1–10s)
- Layout is predictable and structured (lists, cards, tables, dashboards)
- Content type is known ahead of time

### When NOT to Use

- Load < 1s (causes annoying flash)
- Load > 10s (use progress bar instead)
- Content structure is unpredictable
- Single small component (use spinner)

### Anatomy

1. **Match the final layout** — same widths, heights, spacing, gaps
2. **Use appropriate shapes** — rectangles for images, lines for text (varying widths), circles for avatars
3. **Show hierarchy** — larger elements for headings, smaller for body text
4. **Keep it low-fidelity** — too much detail defeats the purpose

### Shimmer Animation (CSS)

```css
.skeleton {
  background: linear-gradient(
    90deg,
    hsl(var(--skeleton-base)) 0%,
    hsl(var(--skeleton-highlight)) 50%,
    hsl(var(--skeleton-base)) 100%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
  border-radius: 4px;
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* Respect reduced motion preference */
@media (prefers-reduced-motion: reduce) {
  .skeleton {
    animation: none;
    opacity: 0.7;
  }
}
```

### Skeleton Component Pattern

```svelte
<script lang="ts">
  interface Props {
    rows?: number;
    showAvatar?: boolean;
    showImage?: boolean;
  }

  let { rows = 3, showAvatar = false, showImage = false }: Props = $props();
</script>

<div class="space-y-4" aria-busy="true" aria-label="Loading content">
  {#if showAvatar}
    <div class="skeleton h-10 w-10 rounded-full" />
  {/if}
  {#if showImage}
    <div class="skeleton h-48 w-full rounded-lg" />
  {/if}
  <div class="skeleton h-6 w-3/5" />
  {#each Array(rows) as _, i}
    <div class="skeleton h-4" style="width: {100 - i * 15}%" />
  {/each}
</div>
```

### Skeleton Table Pattern

```svelte
<div aria-busy="true" aria-label="Loading table data">
  <!-- Header -->
  <div class="flex gap-4 border-b pb-3 mb-3">
    {#each Array(columns) as _}
      <div class="skeleton h-4 flex-1" />
    {/each}
  </div>
  <!-- Rows -->
  {#each Array(rows) as _}
    <div class="flex gap-4 py-3 border-b border-surface-200">
      {#each Array(columns) as _, j}
        <div class="skeleton h-4 flex-1" style="width: {j === 0 ? '80%' : '60%'}" />
      {/each}
    </div>
  {/each}
</div>
```

---

## Spinners

### Delayed Spinner (Avoids Flash)

```svelte
<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    delay?: number;
    size?: 'sm' | 'md' | 'lg';
    label?: string;
  }

  let { delay = 200, size = 'md', label = 'Loading' }: Props = $props();
  let visible = $state(false);

  onMount(() => {
    const timeout = setTimeout(() => { visible = true; }, delay);
    return () => clearTimeout(timeout);
  });

  const sizes = { sm: 'h-4 w-4', md: 'h-8 w-8', lg: 'h-12 w-12' } as const;
</script>

{#if visible}
  <div role="status" aria-live="polite" class="flex items-center justify-center">
    <svg class="{sizes[size]} animate-spin" aria-hidden="true" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" opacity="0.25" />
      <path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
    </svg>
    <span class="sr-only">{label}</span>
  </div>
{/if}
```

### Size Conventions

| Size | Pixels | Use |
|------|--------|-----|
| `sm` (16px) | Inline — buttons, input fields, list items |
| `md` (32px) | Component-level — cards, modals, sections |
| `lg` (48px) | Full-page — primary loading state |

---

## Progress Bars

### Determinate (Known Duration)

Use for: file uploads, multi-step processes, exports.

```svelte
<script lang="ts">
  interface Props {
    value: number;
    max?: number;
    label?: string;
  }

  let { value, max = 100, label }: Props = $props();
  let percentage = $derived(Math.round((value / max) * 100));
</script>

<div class="w-full">
  {#if label}
    <div class="flex justify-between text-sm mb-1">
      <span>{label}</span>
      <span class="text-surface-500">{percentage}%</span>
    </div>
  {/if}
  <div class="h-2 bg-surface-200 rounded-full overflow-hidden"
       role="progressbar" aria-valuenow={value} aria-valuemin={0} aria-valuemax={max}>
    <div class="h-full bg-primary-500 rounded-full transition-[width] duration-300"
         style="width: {percentage}%" />
  </div>
</div>
```

### Indeterminate (Unknown Duration)

Use for: API calls, database queries, processing.

```css
.progress-indeterminate::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--color-primary);
  border-radius: inherit;
  animation: indeterminate 1.5s ease-in-out infinite;
  transform-origin: left;
}

@keyframes indeterminate {
  0% { transform: translateX(-100%) scaleX(0.3); }
  50% { transform: translateX(0%) scaleX(0.5); }
  100% { transform: translateX(100%) scaleX(0.3); }
}
```

---

## Loading Overlay

Block interaction for: form submissions, destructive actions, financial transactions.

```svelte
<script lang="ts">
  interface Props {
    show: boolean;
    message?: string;
    blur?: boolean;
  }

  let { show, message = 'Processing...', blur = true }: Props = $props();
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
       class:backdrop-blur-sm={blur}
       aria-modal="true" role="dialog" aria-label={message}>
    <div class="bg-surface-50 dark:bg-surface-800 p-6 rounded-lg shadow-xl flex flex-col items-center gap-3">
      <svg class="h-10 w-10 animate-spin text-primary-500" aria-hidden="true" viewBox="0 0 24 24" fill="none">
        <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" opacity="0.25" />
        <path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
      </svg>
      <p class="font-medium text-surface-700 dark:text-surface-200">{message}</p>
    </div>
  </div>
{/if}
```

---

## Error States

### Inline Error (Fields)

```svelte
{#if error}
  <p class="text-sm text-error-500 mt-1" role="alert">{error}</p>
{/if}
```

### Error Card (Section-Level)

```svelte
<script lang="ts">
  import { AlertCircle } from 'lucide-svelte';

  interface Props {
    title?: string;
    message: string;
    onretry?: () => void;
  }

  let { title = 'Something went wrong', message, onretry }: Props = $props();
</script>

<div class="flex flex-col items-center py-12 text-center" role="alert">
  <AlertCircle size={48} class="text-error-400 mb-3" />
  <h3 class="text-lg font-semibold mb-1">{title}</h3>
  <p class="text-surface-500 max-w-sm mb-4">{message}</p>
  {#if onretry}
    <button class="btn variant-filled-primary" onclick={onretry}>Try Again</button>
  {/if}
</div>
```

### Error Message Structure

Always include four elements:

1. **What happened:** "Failed to save your changes"
2. **Why (if known):** "Network connection lost"
3. **What to do:** "Check your connection and try again"
4. **Action:** [Retry] or [Contact Support]

Never show: stack traces, database codes, internal system messages.

---

## Empty States

### Empty State Component

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    description?: string;
    icon?: typeof import('lucide-svelte').Icon;
    actions?: Snippet;
  }

  let { title, description, icon: Icon, actions }: Props = $props();
</script>

<div class="flex flex-col items-center py-16 text-center">
  {#if Icon}
    <Icon size={48} class="text-surface-300 mb-4" />
  {/if}
  <h2 class="text-xl font-semibold mb-1">{title}</h2>
  {#if description}
    <p class="text-surface-500 max-w-md mb-6">{description}</p>
  {/if}
  {#if actions}
    <div class="flex gap-3">
      {@render actions()}
    </div>
  {/if}
</div>
```

### Tone by Type

| Type | Tone | Example |
|------|------|---------|
| First-time | Encouraging | "Welcome! Create your first service call" |
| No results | Helpful | "No matches. Try different search terms" |
| Filtered empty | Neutral | "No items match these filters" |
| All cleared | Positive | "All caught up! No pending tasks" |

---

## SvelteKit Patterns

### {#await} with Skeleton

```svelte
{#await dataPromise}
  <SkeletonTable rows={10} columns={5} />
{:then data}
  <DataTable {data} />
{:catch error}
  <ErrorCard message={error.message} onretry={() => dataPromise = refetch()} />
{/await}
```

### Streaming (Deferred Data)

```typescript
// +page.server.ts
export async function load() {
  const critical = await fetchCriticalData();
  return {
    critical,
    deferred: fetchSlowData() // NOT awaited — streams to client
  };
}
```

```svelte
<!-- +page.svelte -->
<CriticalSection data={data.critical} />

{#await data.deferred}
  <SkeletonCards count={4} />
{:then deferred}
  <DeferredSection {deferred} />
{/await}
```

### Enhanced Form Submission

```svelte
<script lang="ts">
  import { enhance } from '$app/forms';
  let submitting = $state(false);
</script>

<form method="POST" use:enhance={() => {
  submitting = true;
  return async ({ update }) => {
    await update();
    submitting = false;
  };
}}>
  <!-- fields -->
  <button type="submit" disabled={submitting}>
    {#if submitting}
      <Spinner size="sm" delay={0} /> Saving...
    {:else}
      Save Changes
    {/if}
  </button>
</form>
```

### Navigation Loading Bar

```svelte
<script lang="ts">
  import { navigating } from '$app/stores';
</script>

{#if $navigating}
  <div class="fixed top-0 left-0 right-0 h-0.5 bg-primary-500 z-50
              animate-[indeterminate_1.5s_ease-in-out_infinite]" />
{/if}
```

---

## Accessibility Checklist

- [ ] Loading container has `aria-busy="true"`
- [ ] Status announcements use `aria-live="polite"` with `role="status"`
- [ ] Spinner SVG has `aria-hidden="true"`, sr-only text provides label
- [ ] Error states use `role="alert"`
- [ ] Progress bars have `role="progressbar"` + `aria-valuenow/min/max`
- [ ] Focus moves to new content heading after async load (for modals/sections)
- [ ] Animations respect `prefers-reduced-motion: reduce`
- [ ] Color is not the only indicator of state (icons + text accompany color)

---

## Sources

- [Nielsen Norman Group — Response Time Limits](https://www.nngroup.com/articles/response-times-3-important-limits/)
- [Nielsen Norman Group — Skeleton Screens 101](https://www.nngroup.com/articles/skeleton-screens/)
- [Material Design 3 — Progress Indicators](https://m3.material.io/components/progress-indicators/guidelines)
- [Apple HIG — Progress Indicators](https://developer.apple.com/design/human-interface-guidelines/progress-indicators)
- [Shopify Polaris — Loading Patterns](https://polaris.shopify.com/components/feedback-indicators/loading)
- [GitHub Primer — Skeleton Loaders](https://primer.style/product/components/skeleton-loaders/)
- [MDN — ARIA: aria-busy](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-busy)
- [SvelteKit — Streaming](https://svelte.dev/docs/kit/load#Streaming-with-promises)

---
name: auto-svelte
description: "Svelte 5 gotchas and quality patterns: SSR state safety, svelte-ignore per-line rules, $state.raw for performance, type quirks with lucide-svelte and Buffer, and $effect discipline. Use when writing Svelte 5 components, SvelteKit routes, or debugging Svelte-specific issues. Triggers: svelte, sveltekit, $state, $derived, $effect, $props, .svelte, rune, +page, +layout, +server, store, .svelte.ts, snippet, load function, form action, handleError."
---

# Svelte 5 — Gotchas & Quality Patterns

Claude already writes correct Svelte 5. This skill covers the non-obvious traps that waste debugging time and the performance patterns Claude doesn't always reach for.

## SSR Global State Leak (Security-Critical)

Module-level `$state()` in `.svelte.ts` files is shared across ALL server requests. User A's data leaks to User B.

```typescript
// DANGEROUS — shared across requests on server
// lib/stores/user.svelte.ts
export let currentUser = $state<User | null>(null); // LEAKS BETWEEN USERS

// SAFE — context is per-request
// lib/state.svelte.ts
export function createAppState() {
  let user = $state<User | null>(null);
  return {
    get user() { return user; },
    set user(v) { user = v; }
  };
}

// +layout.svelte — instantiated per request
import { setContext } from 'svelte';
setContext('app', createAppState());
```

Module-level `$state` is fine for client-only singletons (browser-only stores guarded by `import { browser } from '$app/environment'`). It's dangerous when the `.svelte.ts` file runs on the server.

## svelte-ignore: One Comment Per Line

The `svelte-ignore` directive only covers the **immediately next line**. Multiple warnings need separate comments:

```svelte
// svelte-ignore state_referenced_locally
let name = $state(data.settings?.name ?? '');
// svelte-ignore state_referenced_locally
let email = $state(data.settings?.email ?? '');
```

A single comment before both lines only suppresses the warning for `name`, not `email`.

## $state.raw for Large Collections

`$state()` makes objects deeply reactive by wrapping every nested property in a proxy. For large arrays of objects (100+ items) that you replace wholesale rather than mutate, use `$state.raw()` to skip the proxy overhead:

```svelte
<script lang="ts">
  // 500 items from an API — no need for deep reactivity
  let products = $state.raw<Product[]>([]);

  async function refresh() {
    products = await fetchProducts(); // Reassignment triggers update
    // products.push(item); // Does NOT trigger — must reassign
  }
</script>
```

Use `$state.raw` when: (1) data comes from an API and gets replaced, not mutated, (2) arrays have 100+ items, (3) you notice sluggish reactivity in dev tools.

## $state.snapshot Before Sending to APIs

`$state()` proxied objects break `JSON.stringify` and external APIs. Use `$state.snapshot()` to get a plain object:

```typescript
let formData = $state({ name: '', email: '' });

async function submit() {
  // $state proxy can cause issues with structuredClone, postMessage, etc.
  const plain = $state.snapshot(formData);
  await fetch('/api/users', {
    method: 'POST',
    body: JSON.stringify(plain)
  });
}
```

## Type Quirks

**lucide-svelte icons** don't match `Component<IconProps>`. Use `typeof`:
```svelte
<script lang="ts">
  import { TrendingUp } from 'lucide-svelte';
  interface Props { icon?: typeof TrendingUp; }
</script>
```

**Buffer is not BodyInit** in SvelteKit responses. Wrap it:
```typescript
return new Response(new Uint8Array(buffer), {
  headers: { 'Content-Type': 'application/pdf' }
});
```

## $effect Discipline

`$effect` is for **side effects only** (DOM, network, external systems). If you're computing a value, use `$derived`. This isn't style — `$derived` works during SSR, `$effect` doesn't.

```svelte
<!-- WRONG: effect to compute a value -->
let count = $state(0);
let doubled = $state(0);
$effect(() => { doubled = count * 2; }); // Breaks SSR, creates timing issues

<!-- RIGHT: $derived is pure, SSR-safe, and memoized -->
let doubled = $derived(count * 2);
```

If you must read and write the same `$state` in an `$effect`, use `untrack()` on the read to prevent infinite loops:

```typescript
import { untrack } from 'svelte';

$effect(() => {
  const current = untrack(() => count);
  // safe to use `current` without creating a dependency
});
```

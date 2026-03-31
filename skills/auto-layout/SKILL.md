---
name: auto-layout
description: "Layout and CSS discipline: card restraint, grid over flex for 2D, design token reuse, dead element elimination, viewport units, z-index management, flex truncation, and transition specificity. Corrects card addiction, div wrappers around single elements, hardcoded values that duplicate tokens, and flex-everything layouts. Framework-agnostic with HTML/CSS primary examples and React Native equivalents. Use when building layouts, styling components, writing CSS, or creating UI structure. Triggers: layout, css, style, grid, flex, card, wrapper, div, padding, margin, spacing, z-index, transition, responsive, viewport, height, tailwind, stylesheet, View, StyleSheet."
---

# Layout & CSS — What Claude Gets Wrong

You wrap everything in cards, use flexbox for 2D layouts, hardcode values that exist as design tokens, and leave dead wrapper elements everywhere. In production: designs drift from the system, layouts break at intermediate widths, and every component has 3 unnecessary nesting levels.

## The Eight Rules

1. **Cards are for repeated/grouped content only** — not for page sections
2. **Grid for 2D, flex for 1D** — stop using flex-wrap + calc
3. **Reference every token** — if a CSS variable or theme token exists, use it
4. **Every element earns its place** — no wrapper around a single child
5. **`min-height: 100dvh`** — never `height: 100vh`
6. **Z-index needs a system** — `isolation: isolate` + named scale
7. **Flex children need `min-width: 0`** for truncation to work
8. **Transition specific properties** — never `transition: all`

## Anti-Patterns

| Anti-pattern | What you do | Fix |
|---|---|---|
| Card addiction | Wrap every section in a card with shadow/border/radius | Cards for repeated items and grouped data only. Sections, dividers, or nothing for the rest |
| Flex-everything | `display: flex; flex-wrap: wrap` + `calc()` width hacks for grids | `display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr))` |
| Token ignorance | `color: #131316; padding: 1.5rem; border: 1px solid rgba(255,255,255,0.08)` | `color: var(--card-bg); padding: var(--spacing-lg); border: 1px solid var(--border-color)` |
| Dead wrappers | `<div class="header-left"><h1>Title</h1></div>` | `<h1>Title</h1>` — the wrapper does nothing |
| 100vh | `height: 100vh` — breaks on mobile (content behind browser chrome) | `min-height: 100dvh` with `100vh` fallback |
| Z-index war | `z-index: 10`, `z-index: 100`, `z-index: 9999` scattered everywhere | Named scale via tokens + `isolation: isolate` on component roots |
| Broken truncation | `text-overflow: ellipsis` on flex child without `min-width: 0` | Add `min-width: 0` (or `overflow: hidden`) on the flex child |
| transition: all | `transition: all 0.2s ease` — animates every property change | `transition: background-color 0.15s ease, transform 0.15s ease` |

## Card Restraint

```
USE a card when:                    DO NOT use a card when:
─────────────────────────────       ─────────────────────────────
• Repeated items in a list/grid     • Page-level sections
• Grouped related data              • Single standalone elements
• Interactive clickable units       • Wrapping a heading or text block
• Dashboard metrics (grid of N)     • Testimonial quotes (use <blockquote>)
• Content that moves/reorders       • Settings groups (use fieldset/section)
```

## Grid vs Flex Decision

```
Need columns AND rows?  ──→  Grid
Repeated items?         ──→  Grid (auto-fit/auto-fill + minmax)
Responsive columns?     ──→  Grid (not flex-wrap + calc)
Inline alignment only?  ──→  Flex (header bar, icon + label, button group)
Single axis spacing?    ──→  Flex with gap
```

```css
/* WRONG: flex-wrap with calc width hacks */
.features { display: flex; flex-wrap: wrap; gap: 1rem; }
.feature { width: calc(33.333% - 0.67rem); }

/* RIGHT: grid handles columns, wrapping, and gaps */
.features { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; }
```

## Token Reuse

Before writing ANY value, check the project's design tokens:

```css
/* WRONG — every value hardcoded despite tokens existing */
.panel {
  background: #131316;
  color: #e8e8ec;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0;
}

/* RIGHT — references the design system */
.panel {
  background: var(--card-bg);
  color: var(--text-primary);
  padding: var(--spacing-lg);
  border: 1px solid var(--border-color);
  border-radius: 0;
}
```

In Tailwind with custom tokens: use `bg-surface`, `text-content`, `border-border`, `rounded-card` — not `bg-white`, `text-gray-900`, `border-gray-200`, `rounded-xl`.

**Rule:** If you type a hex color, pixel value, or spacing literal, search the project's tokens first. If a match exists, use the token.

## Dead Element Elimination

Every HTML element must justify its existence. If removing it changes nothing, delete it.

```svelte
<!-- WRONG: wrappers around single children -->
<div class="header-section">
  <div class="header-inner">
    <h1>Dashboard</h1>
  </div>
</div>
<div class="save-area">
  <div class="save-wrapper">
    <button>Save</button>
  </div>
</div>

<!-- RIGHT: flat structure -->
<h1>Dashboard</h1>
<button class="save">Save</button>
```

**Common offenders:**
- `.card-inner` inside `.card` — one element does both jobs
- `.section-header` + `.section-body` when the section has no visual header/body split
- `.foo-wrapper` / `.foo-container` around a single child
- `<div>` wrapping a `<button>` just for `text-align: right` — use `margin-left: auto` on the button

## Viewport Units

```css
/* WRONG — 100vh on mobile = content behind browser chrome */
.hero { height: 100vh; }

/* RIGHT — dvh adapts to browser chrome visibility */
.hero { min-height: 100dvh; }

/* With fallback for older browsers */
.hero {
  min-height: 100vh;
  min-height: 100dvh;
}
```

Always `min-height` over `height` — content can exceed the viewport.

## Z-Index System

```css
/* WRONG — arbitrary escalating values */
.dropdown { z-index: 10; }
.modal-overlay { z-index: 999; }
.modal { z-index: 1000; }
.tooltip { z-index: 9999; }

/* RIGHT — named scale + isolation */
:root {
  --z-dropdown: 1;
  --z-sticky: 2;
  --z-overlay: 3;
  --z-modal: 4;
  --z-tooltip: 5;
}

/* Scope stacking contexts to component roots */
.component-root { isolation: isolate; }
```

Z-index is relative to the nearest stacking context. Use `isolation: isolate` so components don't leak z-index into the global scope.

## Flex Truncation

```css
/* WRONG — truncation won't work, flex child min-width is auto */
.nav-item { display: flex; align-items: center; gap: 0.5rem; }
.nav-label { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

/* RIGHT — min-width: 0 lets the flex child shrink below content width */
.nav-item { display: flex; align-items: center; gap: 0.5rem; }
.nav-label { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
```

Same applies to grid children. Whenever you use `text-overflow: ellipsis` inside flex/grid, add `min-width: 0`.

## Transition Specificity

```css
/* WRONG — animates every property including layout */
.card { transition: all 0.2s ease; }

/* RIGHT — only animate what changes */
.card { transition: background-color 0.15s ease, transform 0.15s ease; }
```

`transition: all` causes: unexpected animations on layout reflows, performance issues from animating geometry properties, visual artifacts when responsive breakpoints change background/padding/width.

## Image Dimensions

Always include `width` and `height` on images to prevent Cumulative Layout Shift:

```html
<!-- WRONG — no dimensions, page jumps when image loads -->
<img src={product.image} alt={product.name} />

<!-- RIGHT — browser reserves space before load -->
<img src={product.image} alt={product.name} width="400" height="300" class="w-full h-auto" />
```

For responsive images, combine HTML attributes (for aspect ratio calculation) with CSS sizing (`width: 100%; height: auto`).

## React Native Equivalents

The same principles apply in React Native — different API, same anti-patterns:

| Web Anti-pattern | React Native Equivalent | Fix |
|---|---|---|
| Card addiction | Wrapping everything in styled `<View>` with shadow/border | Only use card-styled Views for repeated/grouped items |
| Flex-everything (N/A) | RN is flex-only (no grid) — acceptable, but avoid deep nesting | Flatten view hierarchy, use `flexWrap` sparingly |
| Token ignorance | `style={{ padding: 16, color: '#333' }}` hardcoded | Reference theme: `style={{ padding: theme.spacing.md, color: theme.colors.textPrimary }}` |
| Dead wrappers | `<View><View><Text>Hi</Text></View></View>` | Remove intermediate Views that add no style or layout |
| Broken truncation | `<Text numberOfLines={1}>` inside flex without `flex: 1` or `flexShrink: 1` | Add `flex: 1` or `flexShrink: 1` on the Text to allow shrinking |
| Style duplication | Inline `style={{...}}` objects recreated every render | Use `StyleSheet.create()`, reference theme tokens |

```typescript
// WRONG — hardcoded values, unnecessary wrapper
const Card = ({ title }: { title: string }) => (
  <View style={{ padding: 16, backgroundColor: '#1a1a1f', borderRadius: 8 }}>
    <View style={{ flexDirection: 'row', alignItems: 'center' }}>
      <View style={{ flex: 1 }}>
        <Text style={{ color: '#e8e8ec', fontSize: 16 }}>{title}</Text>
      </View>
    </View>
  </View>
);

// RIGHT — theme tokens, flat structure
const Card = ({ title }: { title: string }) => (
  <View style={[styles.card, { backgroundColor: theme.colors.cardBg }]}>
    <Text style={[styles.cardTitle, { color: theme.colors.textPrimary }]}>{title}</Text>
  </View>
);

const styles = StyleSheet.create({
  card: { padding: theme.spacing.md },
  cardTitle: { fontSize: theme.typography.body },
});
```

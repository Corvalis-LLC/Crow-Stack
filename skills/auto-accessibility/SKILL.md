---
name: auto-accessibility
description: "Accessibility patterns Claude inconsistently applies: ARIA role completeness, touch target sizing, forced-colors/reduced-motion support, and WCAG 2.2 new criteria. Use when building UI components, forms, modals, tables, or interactive features. Triggers: accessibility, a11y, accessible, WCAG, screen reader, keyboard navigation, focus, aria, role, alt text, contrast, tab index, focus trap, landmark, semantic HTML, label, fieldset, live region."
---

# Accessibility — Patterns Claude Skips

You already know WCAG 2.2 AA. This skill covers the patterns you inconsistently apply.

## ARIA completeness

Never skip ARIA roles and attributes on interactive elements. Every interactive component needs the full set — not just the obvious ones.

- **Modals:** `role="dialog"` (or `role="alertdialog"` for destructive confirmations) + `aria-modal="true"` + `aria-labelledby` pointing to the title + `tabindex="-1"` on the dialog container
- **Forms:** `aria-required` on required inputs, `aria-invalid` when validation fails, `aria-describedby` switching between hint text and error message IDs depending on state
- **Icon buttons:** Always `aria-label` — never ship an icon button without one
- **Tables:** `<caption>`, `scope="col"`, `aria-sort` on sortable columns, `aria-live="polite"` region to announce sort changes
- **Loading states:** `aria-busy="true"` on the updating region, `role="status"` for non-intrusive updates
- **Errors:** `role="alert"` or `aria-live="assertive"` — errors must be announced, not just visually displayed

## Touch targets

WCAG 2.2 AA requires 24x24px minimum. Apple/Google recommend 44x44px. Always use `min-height: 44px` on buttons, links, and interactive controls. Add `gap: 8px` minimum between adjacent targets.

## Media queries you skip

Always include these on interactive components:

```css
@media (prefers-reduced-motion: reduce) {
  * { animation-duration: 0.01ms !important; transition-duration: 0.01ms !important; }
}
@media (forced-colors: active) {
  :focus-visible { outline: 3px solid CanvasText; }
}
```

## Focus management

- Focus trap in modals (Tab cycles within, Shift+Tab reverses)
- Focus restoration: return focus to the trigger element when a modal/popover closes
- `scroll-margin-top` on focusable elements so sticky headers don't obscure them
- Never `outline: none` without a visible replacement

## Svelte-specific gotchas

- Interactive ARIA roles (`menu`, `dialog`, `alertdialog`, `toolbar`) require `tabindex="-1"` on the container — Svelte warns if missing
- Don't add redundant roles to native HTML (`<table role="table">`, `<nav role="navigation">` — already implicit)
- Use `<span>` not `<label>` for read-only display text with no associated control

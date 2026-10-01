# Layout implementation guardrails

Apply these rules when editing CSS or native layout, alongside the relevant Impeccable playbook. They preserve the implementation guidance from the retired `auto-layout` skill without a second design workflow.

- Use CSS Grid for coordinated rows and columns; flex for a single axis. Avoid flex-wrap with calculated widths when Grid expresses the layout directly.
- Search the project's tokens before adding color, spacing, radius, typography, or layer values. Preserve the established scale.
- For content that fills a mobile viewport, prefer `min-height: 100dvh` with an appropriate fallback. Fixed viewport height must not clip content or place controls behind browser chrome.
- Use the project's named z-index scale. Introduce `isolation: isolate` at intentional component stacking boundaries; check overlays and portals against their actual stacking context.
- Give shrinking flex/grid children `min-width: 0` when their content must truncate. Verify overflow with long real content.
- Transition only the properties that should animate. Avoid `transition: all`; verify intentional reduced-motion alternatives.
- Reserve image space with width/height or an aspect ratio to prevent layout shifts. Remove wrappers only when they provide no semantic, layout, accessibility, or interaction purpose.
- React Native has no CSS Grid: use its flex layout, theme tokens, and shallow view hierarchy. Truncating text may need `flex: 1` or `flexShrink: 1` with `numberOfLines`; use the project's StyleSheet conventions.

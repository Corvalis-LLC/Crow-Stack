# Design Principles

Cross-cutting patterns sourced from Apple HIG, shadcn/ui, Stripe, Linear, Vercel Geist, GitHub Primer, and React Bits. These apply regardless of which design set is active.

## Core Principles

- **Clarity**: Interface instantly understandable. Legible text, purposeful icons, strong visual hierarchy, focus on essentials. (Apple HIG)
- **Deference**: UI steps back, content shines. Neutral backgrounds, subtle chrome. Content is the hero. (Apple HIG)
- **Depth**: Visual layering through shadows and elevation. Logical navigation with clear paths. (Apple HIG)
- **Consistency**: Same gesture = same result everywhere. Reuse components. Consistent terminology. Predictable placement.

## Creative Direction

Aligned with [Claude Code Frontend Design](https://github.com/anthropics/claude-cookbooks/blob/main/coding/prompting_for_frontend_aesthetics.ipynb) — the official Anthropic guidelines for avoiding generic AI output.

### Design Thinking

Before coding, understand the context and commit to a **bold aesthetic direction**:

1. **Purpose**: What problem does this interface solve? Who uses it?
2. **Tone**: Pick a clear direction — brutally minimal, maximalist chaos, retro-futuristic, organic/natural, luxury/refined, playful/toy-like, editorial/magazine, brutalist/raw, art deco/geometric, soft/pastel, industrial/utilitarian. Use these for inspiration but design one true to the aesthetic. (Claude Code Frontend Design)
3. **Constraints**: Framework, performance, accessibility, existing design tokens.
4. **Differentiation**: What makes this **unforgettable**? What's the one thing someone will remember? (Claude Code Frontend Design)

### Anti-AI Slop (CRITICAL — from Claude Code Frontend Design)

**NEVER produce generic AI-generated aesthetics.** This is the single most important design rule. Every output must feel genuinely designed for its context, not generated.

| Anti-Pattern | Why It's Slop | Instead |
|--------------|---------------|---------|
| Overused fonts without thought (Inter, Roboto, Arial, system fonts, Space Grotesk) | Convergent — every AI picks these | Choose fonts that are beautiful, unique, and interesting. Pair a distinctive display font with a refined body font. |
| Cliched color schemes (purple gradients on white) | The "AI default" palette | Commit to a cohesive palette. Dominant colors with sharp accents outperform timid, evenly-distributed palettes. |
| Predictable, identical layouts | Cookie-cutter template feel | Unexpected layouts. Asymmetry. Overlap. Diagonal flow. Grid-breaking elements. Generous negative space OR controlled density. |
| Flat solid-color backgrounds | Lifeless, unfinished | Atmosphere and depth — gradient meshes, noise textures, geometric patterns, layered transparencies, dramatic shadows, grain overlays. |
| Scattered micro-interactions | Busy without purpose | One well-orchestrated page entrance with staggered reveals creates more delight than scattered effects. |
| Emojis or unicode symbols in UI | Inconsistent rendering, unprofessional | Icon components (Lucide) always. |

**Critical instruction** (from Claude Code Frontend Design): "Interpret creatively and make unexpected choices that feel genuinely designed for the context. No design should be the same. Vary between light and dark themes, different fonts, different aesthetics. NEVER converge on common choices across generations."

### Frontend Aesthetics (from Claude Code Frontend Design)

When building new pages or creative components, focus on:

- **Typography**: Choose fonts that are beautiful and interesting, not generic defaults. Unexpected, characterful choices.
- **Color & Theme**: CSS variables for consistency. Dominant + accent > timid distribution.
- **Motion**: Prioritize CSS-only solutions. Focus budget on high-impact moments: page load reveals (animation-delay stagger), scroll-triggers, hover surprises.
- **Spatial Composition**: Unexpected layouts. Asymmetry. Overlap. Grid-breaking elements.
- **Backgrounds & Visual Details**: Atmosphere over solid colors. Textures, gradients, patterns, grain.

### Matching Complexity to Vision
- **Maximalist designs** need elaborate code — extensive animations, layered effects, rich details
- **Minimalist designs** need restraint and precision — careful spacing, typography, subtle details
- Elegance comes from executing the vision well, not from adding more

### When Anti-Slop Rules Relax

The anti-slop rules above apply at **full strength** for creative/marketing/public-facing pages. For **admin tools and data-dense internal UI** (which is most of the Dadson app), practical choices take priority:
- System fonts and Inter ARE appropriate for admin dashboards — readability > personality
- Consistent layouts ARE correct for data tables and forms — predictability > surprise
- Solid backgrounds ARE fine for functional tooling — density > atmosphere
- The active design set's tokens override anti-slop defaults when they conflict

## Visual Hierarchy Formula
```
Size + Weight + Color + Space = Importance
```
- **Primary**: Largest, boldest, accent color, most surrounding space
- **Secondary**: Medium size, semibold, default color, moderate space
- **Tertiary**: Smallest, normal weight, muted color, tight space

### Hierarchy Rules
1. One focal point per view — don't make users guess
2. Squint test — blur your vision; hierarchy should still be clear
3. Size ratios — headlines 1.25-1.5x larger minimum
4. Weight creates emphasis — bold for titles, regular for body
5. Color guides attention — primary color for CTAs, muted for secondary

## Typography — Sourced Patterns

### Modular Scales (choose one per project)

From Apple HIG, Stripe, and Geist research:

| Ratio | Name | Character | Source |
|-------|------|-----------|--------|
| 1.125 | Major Second | Dense, tight — max density | Primer |
| 1.2 | Minor Third | Systematic, precise — data tools | Apple HIG |
| 1.25 | Major Third | Balanced, readable — general apps | Stripe |
| 1.333 | Perfect Fourth | Dramatic, bold — marketing | Geist |

### Font Weight Contrast

Different weight spreads create different moods. Sourced from real systems:

| Pattern | Body | Heading | Hero | Character | Source |
|---------|------|---------|------|-----------|--------|
| Light/Medium | 300 | 500 | 500 | Elegant, premium | Stripe |
| Normal/SemiBold | 400 | 600 | 700 | Professional, clear | Apple HIG, shadcn |
| Normal/Bold | 400 | 700 | 800 | Dramatic, high impact | Geist |
| Normal/Normal | 400 | 400 | 300 | Dense, understated | Primer (.f00-.f2 light) |

### Letter Spacing Rules
- Body text: default (0)
- Headings >= 24px: tighten -0.02em (Geist, Stripe)
- Hero >= 40px: tighten -0.04em (Geist)
- Small caps / labels: widen +0.05em (Stripe, Linear)
- Tabular numerals: `font-variant-numeric: tabular-nums` for financial data

## Color — Sourced Patterns

### Color Space Approaches

| Approach | How | Source |
|----------|-----|--------|
| HSL-based tokens | Easy manipulation, familiar | shadcn/ui (OKLCh variant) |
| CIELAB perceptual | Uniform brightness across hues | Stripe |
| LCH elevation | Dark mode surfaces via lightness channel | Linear |
| Functional tokens | Semantic names → theme-dependent values | Primer |

### Gray Scale Character

Grays are not neutral — they carry personality:

| Type | Example | Mood | Source |
|------|---------|------|--------|
| Pure grays | #111, #333, #666, #999, #eaeaea | Clean, sharp, Swiss | Geist |
| Blue-tinted | #1f2328, #656d76, #8c959f, #d0d7de | Approachable, developer | Primer |
| Warm blue-gray | #30313d, #425466, #697386 | Premium, trustworthy | Stripe |
| Zinc (OKLCh) | oklch(0.141 0.005 285.82) scale | Systematic, modern | shadcn/ui |

### Semantic Color Guidelines
- **Never use color alone** — always pair with icon + text
- **Red = destructive/error** — consistent across all systems
- **Blue = primary action** — universal convention
- **Green = success** — universal convention
- **Orange/amber = warning** — universal convention

## Spacing — Sourced Patterns

### Grid Systems

| Base | Scale | Personality | Source |
|------|-------|-------------|--------|
| 4px | 4, 8, 12, 16, 20, 24, 32, 40, 48 | Precise, fine control | Apple HIG, Geist |
| 8px | 0, 4, 8, 16, 24, 32, 40 | Rhythmic, structural | Primer |

### Whitespace Spectrum

| Density | Whitespace % | Card Padding | Body Gap | Source |
|---------|-------------|--------------|----------|--------|
| Maximum | 20-40% | 12-16px | 8px | Primer (utility) |
| Medium | 40-60% | 16-24px | 16px | Apple, Geist |
| Generous | 60-80% | 24-32px | 24-32px | Stripe (warmth) |

## Elevation — Sourced Patterns

Three philosophies — pick one per project:

### 1. Shadow-Based (Stripe pattern)
```css
/* Purple-tinted dual shadows — from Stripe shadow mixin */
--shadow-sm: 0 2px 5px rgba(50,50,93,0.09), 0 1px 2px rgba(0,0,0,0.07);
--shadow-md: 0 4px 6px rgba(50,50,93,0.09), 0 1px 3px rgba(0,0,0,0.08);
--shadow-lg: 0 15px 35px rgba(50,50,93,0.1), 0 5px 15px rgba(0,0,0,0.07);
```
**Character**: Warm, premium, layered depth

### 2. Border-Based (Geist pattern)
```css
/* 1px borders via box-shadow for sub-pixel precision */
--card-shadow: 0 0 0 1px var(--border-color);
--card-shadow-hover: 0 0 0 1px var(--text-muted);
```
**Character**: Clean, flat, Swiss minimalism

### 3. Flat + Overlay (Primer pattern)
```css
/* No shadows by default — shadows only for overlays */
--shadow-resting-small: 0 1px 0 rgba(31, 35, 40, 0.1);
--shadow-floating-small: 0 1px 3px rgba(31, 35, 40, 0.12), 0 8px 24px rgba(66, 74, 83, 0.12);
```
**Character**: Dense, functional, developer-tool

## Motion — Sourced Patterns

### Timing Benchmarks

From Apple HIG Core Animation and React Bits research:

| Duration | Use Case | Easing | Source |
|----------|----------|--------|--------|
| 75-100ms | Hover states, button feedback | ease-out | Primer |
| 150-200ms | Micro-interactions, toggles | ease or ease-in-out | Geist, Apple |
| 200-300ms | Content transitions, panels | ease-out | Stripe |
| 400-500ms | Hero entrances, page transitions | ease-out or spring | React Bits |
| Max 500ms | Never exceed for any UI animation | — | All systems |

### Apple HIG Default Easing
```css
/* Apple Core Animation — standard easing curve */
transition-timing-function: cubic-bezier(0.25, 0.1, 0.25, 1.0);
/* Duration: 250ms default */
```

### Spring Physics (from React Bits)

| Context | Stiffness | Damping | Mass | Feel |
|---------|-----------|---------|------|------|
| Dock magnification | 150 | 12 | 0.1 | Snappy, responsive |
| List items | 500 | 30 | 1 | Quick settle |
| Default (Framer) | 100 | 10 | 1 | Natural, balanced |
| Bouncy | 170 | 5 | 1 | Playful, springy |
| Gentle | 170 | 15 | 1 | Soft, gradual |

### Motion Rules
- Every animation must serve a purpose: feedback, orientation, guidance, continuity
- If it doesn't serve a purpose, remove it
- Never exceed 500ms — feels sluggish
- Always respect `prefers-reduced-motion`
- Prioritize CSS-only solutions (pure CSS animations from React Bits: ShinyText, GlitchText, GradientText)
- Focus budget: page entrance, scroll reveals, hover surprises, state transitions

## Touch Targets — Sourced Spectrum

| Size | Use | Source |
|------|-----|--------|
| 24px | WCAG 2.2 absolute minimum | WCAG |
| 28px | Compact dense UI (acceptable with spacing) | Primer small |
| 32px | Dense default (high-density admin tools) | Primer medium |
| 36px | Standard compact | shadcn/ui h-9 |
| 40px | Standard default | shadcn/ui h-10 |
| 44px | Apple minimum / comfortable touch | Apple HIG, shadcn h-11 |

## Border Radius — Sourced Spectrum

| Style | Values | Character | Source |
|-------|--------|-----------|--------|
| Tight | 4px uniform | Functional, sharp | Primer, Stripe |
| Moderate | 6-8px | Professional, balanced | Geist, Apple |
| Rounded | 10-12px | Friendly, approachable | shadcn (--radius: 0.625rem) |
| Generous | 12-16px | Soft, consumer | Stripe cards |
| Full | 9999px | Pills, badges | All systems |

## CSS Gotchas
- Always include standard `line-clamp` with `-webkit-line-clamp` (prefer Tailwind `line-clamp-*` classes)
- No empty CSS rulesets (even with comments) — triggers warnings
- Icons over emojis — always use Lucide components, never unicode

## Quality Checklist
- [ ] Can a new user understand the interface in 5 seconds?
- [ ] Is the primary action obvious?
- [ ] One clear focal point per view?
- [ ] Does the squint test pass?
- [ ] All buttons have hover/active states?
- [ ] Loading states implemented?
- [ ] Empty states helpful?
- [ ] Color contrast 4.5:1+?
- [ ] Keyboard navigation works?
- [ ] Focus indicators visible?
- [ ] Reduced motion respected?
- [ ] Consistent spacing throughout?
- [ ] Animations under 300ms?
- [ ] Whitespace used intentionally?
- [ ] CSS variables used (no hardcoded hex)?
- [ ] Works in both light and dark themes?

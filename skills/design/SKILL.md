---
name: design
description: "Creative UI/UX design system with curated component cheat sheets, design auditing, component generation, and style migration. Auto-loaded by summon Path D — not a standalone entry point. Use /summon and choose path 4 (Design) for UI/UX work."
---

# Design System Orchestrator

**Entry point:** `/summon` → Path D (Design). This skill is auto-loaded by summon's design path, not invoked directly.

## Auto-Bootstrap (ALWAYS — No Questions)

On EVERY invocation — whether via summon Path D or direct load — silently bootstrap:

1. **Load `auto-layout`** — layout discipline applies to all design work (non-negotiable)
2. Read `.design/system.md` from the **current project root** (next to `CLAUDE.md`)
3. Extract `active_set` from YAML frontmatter
4. Read `.claude/skills/design/sets/{active_set}.md`
5. Apply the design set's tokens, patterns, and cheat sheets to ALL design work
6. **Do NOT ask questions or present menus** — proceed directly with the user's task

If `.design/system.md` doesn't exist, run `--initialize` to guide the user through setup.

## Command Routing

When invoked with explicit `$ARGUMENTS`, route to a subcommand:

| Command | Action |
|---------|--------|
| `(no args)` | Bootstrap silently, apply active set to current task |
| `--initialize` | Guided setup — create `.design/system.md` for this project |
| `--update` | Re-walk initialization questions, update existing `.design/system.md` |
| `status` | Read `.design/system.md`, report active set and stats |
| `set <name>` | Switch active design set, update `.design/system.md` |
| `build <type>` | Dispatch **builder agent** to create a component |
| `audit [path]` | Dispatch **auditor agent** (specific path) or **pattern-matcher** (full scan) |
| `migrate <path> --to=<set>` | Dispatch **migrator agent** to refactor components |

## Initialize & Update

### `/design --initialize` (New Project Setup)

Walk the user through guided questions to create `.design/system.md`. Use `AskUserQuestion` for each step. This is the ONE place where the skill asks questions.

**Step 1 — Project Context**
```
Question: "What kind of project is this?"
Options:
- "Admin / Internal tool" → suggests utility or precision set
- "Consumer SaaS / Collaborative" → suggests warmth set
- "Marketing / Landing pages" → suggests bold set
- "Developer tool / Data-dense" → suggests precision or utility set
```

**Step 2 — Aesthetic Direction**
Reference [Claude Code Frontend Design](https://github.com/anthropics/claude-cookbooks/blob/main/coding/prompting_for_frontend_aesthetics.ipynb) principles:
```
Question: "What aesthetic tone fits this project?"
Options:
- "Professional & functional" — clean, trustworthy, content-first
- "Warm & approachable" — soft shadows, generous space, friendly
- "Bold & dramatic" — high contrast, sharp, conversion-focused
- "Dense & utilitarian" — compact, muted, maximum information
```

**Step 3 — Theme Preference**
```
Question: "Light or dark mode priority?"
Options:
- "Light-first (dark mode secondary)"
- "Dark-first (light mode secondary)"
- "Both equally important"
```

**Step 4 — Typography Personality**
```
Question: "What typography character?"
Options:
- "System fonts (fast, practical)" — Inter, system-ui stack
- "Distinctive display + clean body" — characterful headings, readable body
- "Monospace-accented" — code/developer aesthetic
- "Let the design set decide"
```

**Step 5 — Density**
```
Question: "How dense should the UI be?"
Options:
- "Compact — maximum information visible" → tight spacing, small text
- "Balanced — readable with room to breathe" → moderate spacing
- "Generous — content-first with whitespace" → large spacing, big text
```

**Step 6 — Anti-AI Slop Stance**
Reference Claude Code Frontend Design anti-patterns:
```
Question: "How creative should new pages be?"
Options:
- "Functional first — consistency over creativity (admin tools)"
- "Balanced — distinctive but professional"
- "Bold — every page should feel uniquely designed (creative/marketing)"
```

**After all questions:** Use this decision matrix to select the design set:

| Project Type | Aesthetic | Density | Best Set |
|-------------|-----------|---------|----------|
| Admin/Internal | Professional/Dense | Compact | `utility` |
| Admin/Internal | Professional | Balanced | `precision` |
| Admin/Internal | Warm/Approachable | Balanced/Generous | `warmth` |
| Consumer SaaS | Warm/Approachable | Balanced/Generous | `warmth` |
| Consumer SaaS | Professional | Balanced | `precision` |
| Consumer SaaS | Bold/Dramatic | Any | `bold` |
| Marketing/Landing | Bold/Dramatic | Any | `bold` |
| Marketing/Landing | Warm/Approachable | Generous | `warmth` |
| Developer tool | Professional/Dense | Compact | `utility` |
| Developer tool | Professional | Balanced | `precision` |
| Data-dense | Dense/Utilitarian | Compact | `utility` |
| Data-dense | Professional | Balanced | `precision` |

**Tiebreakers:**
- Dark-first preference → `bold` or `precision`
- Light-first preference → `warmth` or `precision`
- Monospace typography → `utility` or `bold`
- Creative/unique pages → `bold`
- Functional/consistent pages → `precision` or `utility`

Write `.design/system.md` with:
```yaml
---
active_set: <chosen set>
customizations:
  theme_priority: <light|dark|both>
  density: <compact|balanced|generous>
  typography: <system|distinctive|monospace|default>
  creativity: <functional|balanced|bold>
last_audit: null
component_count: 0
initialized: <date>
---
```

Confirm the setup: "Initialized `.design/system.md` with **{set}** design set. Customizations: {summary}. Run `/design --update` anytime to change these."

### `/design --update` (Re-configure Existing Project)

Same questions as `--initialize`, but:
1. Read existing `.design/system.md` first
2. Show current values as the pre-selected defaults in each question
3. Only update fields that changed
4. Preserve `last_audit`, `component_count`, and `audit_history`

Example: "Current theme priority is **dark-first**. Want to change it?" with current value highlighted.

## Active Design Set

### Loading the Active Set
1. Read `.design/system.md` to get `active_set` value
2. Read `.claude/skills/design/sets/{active_set}.md`
3. Use the design set's tokens, patterns, and cheat sheets for ALL design decisions

### Available Sets

**General-purpose (offered during initialization):**
| Set | File | Character |
|-----|------|-----------|
| `precision` | `sets/precision.md` | Apple/shadcn — tight, monochrome, systematic |
| `warmth` | `sets/warmth.md` | Stripe/Linear — soft shadows, generous space, friendly |
| `bold` | `sets/bold.md` | Vercel/Raycast — high contrast, dramatic, sharp |
| `utility` | `sets/utility.md` | GitHub/VS Code — dense, muted, functional |

**Repo-specific (not offered during initialization):**
| Set | File | Character |
|-----|------|-----------|
| `dadson` | `sets/dadson.md` | Dadson App only — professional, accessible, warm blue accent |

**Planned (not yet created):**
| Set | File | Character |
|-----|------|-----------|
| `corvus` | `sets/corvus.md` | Corvalis brand — spectral cyberpunk, brutalist undertones |
| `modern` | `sets/modern.md` | TBD |

### Switching Sets
When user runs `/design set <name>`:
1. Verify the set file exists at `.claude/skills/design/sets/{name}.md`
2. Read it to confirm it loads correctly
3. Update `.design/system.md` — change `active_set` to new value
4. Confirm: "Switched to **{name}** design set. All future design work will use {name} patterns."

## Design Philosophy (Condensed)

- **Professional, not trendy** — clean, functional, trustworthy
- **Accessible by default** — WCAG AA minimum, high contrast, large touch targets (44px+)
- **Distinctive, not generic** — every interface intentionally designed for its context
- **Icons over emojis** — NEVER use emojis or unicode in UI, use Lucide icon components
- **Content first** — UI defers to content, neutral backgrounds, subtle chrome

For full principles, creative direction, and quality checklist, see [principles.md](./principles.md).

## Agent Dispatch

### Audit (targeted)
When user specifies a path: `/design audit <path>`
1. Read `.design/system.md` for active set
2. Read `sets/{active_set}.md` for design rules
3. Read `agents/auditor.md` for agent instructions
4. Launch **Task** with `subagent_type='general-purpose'`, `model='sonnet'`:
   - Prompt = auditor instructions + full design set content + target path
5. Present findings to user with severity levels (VIOLATION / WARNING / SUGGESTION)
6. If user approves fixes, dispatch builder agent for each file

### Audit (full scan)
When user runs `/design audit` with no path:
1. Read active design set (same as above)
2. Read `agents/pattern-matcher.md` for agent instructions
3. Launch **Task** with `subagent_type='general-purpose'`, `model='sonnet'`:
   - Prompt = pattern-matcher instructions + design set + scan scope (src/lib/components/ + src/routes/)
4. Present categorized report: inconsistencies, drift, consolidation candidates, gaps

### Build
When user runs `/design build <component-type>`:
1. Read `.design/system.md` for active set
2. Read `sets/{active_set}.md` for design rules and cheat sheets
3. Read `agents/builder.md` for agent instructions
4. Launch **Task** with `subagent_type='general-purpose'`, `model='sonnet'`:
   - Prompt = builder instructions + design set content + component type + target directory
5. Agent creates the component file
6. Report what was created, show key code highlights

### Migrate
When user runs `/design migrate <path> --to=<target-set>`:
1. Read current active set AND target set
2. Read `agents/migrator.md` for agent instructions
3. Launch **Task** with `subagent_type='general-purpose'`:
   - Prompt = migrator instructions + both design sets + target path
4. Agent edits files in place (visual changes only, preserves logic)
5. Summarize what was changed per file

## Persistence

### Reading State
At the start of any `/design` invocation:
1. Read `.design/system.md`
2. Extract `active_set` from YAML frontmatter
3. Use this to determine which design set to load

### Writing State
After these operations, update `.design/system.md`:
- **Set switch**: Update `active_set`
- **Audit completion**: Update `last_audit` with date and summary
- **Component creation**: Increment `component_count`
- **Customizations**: Record any user-specified overrides

## Per-Project Design State

The `.design/` directory lives in each project root (next to `CLAUDE.md`). Every project can have its own active set, overrides, and audit history. The skill reads from whichever `.design/system.md` is in the current working directory.

```
project-root/
├── .design/
│   └── system.md          # active_set, overrides, audit history
├── .claude/skills/design/  # skill files (shared across projects)
└── CLAUDE.md
```

When switching between projects, the skill automatically picks up that project's `.design/system.md` — no configuration needed.

## Loading & Feedback States

Every data-fetching view and mutation needs a loading/error/empty strategy. Choose the right pattern by duration and context:

| Duration | Pattern | When |
|----------|---------|------|
| < 200ms | Nothing | Perceived as instant — any indicator would flash |
| 200ms–1s | Delayed spinner (200ms delay) | Avoids flash for fast loads |
| 1–10s | Skeleton screen | Predictable layout, reduces perceived wait by 20–30% |
| 1–10s | Spinner | Unknown layout or single component |
| 10s+ | Progress bar + percentage | User needs completion estimate |

### Skeleton Rules

- **Match the final layout exactly** — skeletons that don't match cause layout shift and confusion
- **Use shimmer animation** (1.5s, 90deg gradient) — pulse is acceptable but shimmer is preferred
- **Include `aria-busy="true"`** on the loading container, `aria-live="polite"` for screen reader announcements

### Error States

| Error Type | Display | Auto-dismiss |
|-----------|---------|-------------|
| Field validation | Inline below field | No |
| Action failed | Toast notification | Yes (7s) |
| Network offline | Banner (top) | No |
| 404 / 500 | Full error page | No |
| Session expired | Modal | No |

**Always include:** What happened → Why → What to do → [Action button]

### Empty States

| Type | Illustration? | Tone | CTA |
|------|-------------|------|-----|
| First-time use | Yes | Encouraging | "Create [Item]" |
| No search results | No | Helpful | "Clear filters" |
| Filtered empty | No | Neutral | "Remove filters" |
| All cleared | Optional | Celebratory | "Create more" |

### Overlay Loading

Use overlays that block interaction **only for**: form submissions, destructive actions, financial transactions. Prevent double-submission with disabled state + visual feedback.

For component patterns, SvelteKit `{#await}` blocks, streaming, `use:enhance` loading states, and accessible implementations, see [loading-states.md](./loading-states.md).

## Component Standards (All Sets)

Regardless of which design set is active, ALL components must:
- Use **Svelte 5 runes** (`$state`, `$derived`, `$effect`, `$props`) — never Svelte 4 stores
- Define **TypeScript interfaces** for all props
- Use **CSS variables** from the active design set — never hardcoded hex colors
- Include **ARIA attributes** for accessibility (labels, roles, expanded states)
- Support **keyboard navigation** where interactive
- Work in **both light and dark themes**
- Be **responsive** (mobile-first with `sm:`, `lg:` breakpoints)
- Use **Lucide icons** — never emojis or unicode symbols

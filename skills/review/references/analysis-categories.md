# Analysis Categories

Review changes across these dimensions. Each finding has both a **category** (domain) and a **comment type** (severity of action needed).

## Comment Types

Every finding is tagged with exactly one comment type:

| Type | Action | Display |
|------|--------|---------|
| **Issue** | Something that will or could cause bugs, failures, or vulnerabilities | Always shown prominently |
| **Suggestion** | Improvement for code quality, maintainability, or performance | Shown in main report |
| **Nitpick** | Minor style or convention issue | Collapsed/separate section |

---

## Category 1: Security (CRITICAL)

- SQL injection vulnerabilities (raw string interpolation in queries)
- XSS vulnerabilities (unsanitized `{@html}` in Svelte, `.innerHTML`)
- Missing authentication/authorization checks
- Hardcoded secrets or credentials
- Missing rate limiting on public endpoints
- Insecure session handling
- Missing input validation (no Zod schema)
- Timing-safe comparison not used for secrets

---

## Category 2: Logic & Correctness

- Business logic errors and incorrect conditions
- Missing error handling or silent failures
- Race conditions (concurrent writes, unguarded state)
- Edge cases not handled (empty arrays, null values, zero amounts)
- Off-by-one errors in loops or pagination
- Incorrect async/await usage (unhandled promises)
- Wrong HTTP status codes for error conditions

---

## Category 3: Duplicated Code & Tech Debt

**This is a dedicated analysis pass.** Examine all changed files plus their surrounding context for:

### Code Duplication
- **Identical blocks**: 5+ lines of code that appear verbatim in multiple locations
- **Structural clones**: Same logic with different variable names (would be caught by AST-level analysis)
- **Similar patterns**: 3+ instances of the same pattern that could be extracted to a shared utility

### Tech Debt Signals
- **Dead code**: Unreachable branches, unused exports, commented-out code
- **TODOs without tickets**: `TODO` or `FIXME` comments not linked to an issue tracker
- **Temporary workarounds**: Code marked as "hack", "workaround", "temporary"
- **Magic numbers/strings**: Hardcoded values that should be named constants
- **Growing functions**: Functions over 50 lines that should be split
- **Inconsistent patterns**: Code that does the same thing differently in different places
- **Deprecated usage**: Using APIs or patterns the codebase has moved away from
- **Implicit coupling**: Files that always change together but aren't formally connected

### Report Format for Tech Debt
```markdown
### Tech Debt & Duplication

**Duplicated Code Found:**
- `src/routes/api/users/+server.ts:23-45` duplicates `src/routes/api/clients/+server.ts:30-52`
  - Extract to: `$lib/server/api-helpers.ts` → `validateAndPaginate()`

**Dead Code:**
- `src/lib/utils/legacy.ts` — exported but not imported anywhere

**TODOs Without Tickets:**
- `src/lib/server/services/email/send.ts:67` — "TODO: handle bounce notifications"
```

---

## Category 4: Type Safety

- Use of `any` type
- Missing type annotations on exports
- Unsafe type assertions (`as` without prior validation)
- `@ts-ignore` or `@ts-expect-error` without justification
- Missing null/undefined checks
- Non-exhaustive switch statements on discriminated unions

---

## Category 5: API Design

- Missing error handling in API routes
- Inconsistent response formats
- Missing input validation on request body/params
- No rate limiting on sensitive endpoints
- Missing audit logging for mutations
- Wrong HTTP methods (using GET for mutations)

---

## Category 6: Accessibility

- Missing ARIA labels on interactive elements (icon buttons, links)
- Missing alt text on images
- Insufficient color contrast
- Missing keyboard navigation support
- Focus management issues in modals/dialogs
- Missing form labels or error associations

---

## Category 7: Performance

- N+1 query patterns (loop with database calls)
- Missing database indexes for new query patterns
- Large bundle imports that could be code-split
- Missing pagination on list endpoints
- Unbounded queries (no LIMIT)
- Sequential fetches that could be parallel (`Promise.all`)

---

## Category 8: Cross-File Impact

**Inspired by CodeRabbit's code graph analysis.** For each changed file, check:

- **Imports/exports**: Does the change break any importers? Renamed exports? Removed exports?
- **Interface changes**: Did a type/interface change? Are all consumers updated?
- **API contract changes**: Did request/response shape change? Are all callers updated?
- **Database schema changes**: Did a column/table change? Are all queries updated?
- **Implicit coupling**: Files that typically change together but weren't included in this diff

```markdown
### Cross-File Impact

**Breaking Changes Detected:**
- `UserSchema` in `src/lib/schemas/user.ts` added required field `phone`
  - `src/routes/api/users/+server.ts` — POST handler needs to accept `phone`
  - `src/lib/components/UserForm.svelte` — form needs phone field

**Missing Co-Changes:**
- `src/lib/server/services/vendmaster/commission.ts` changed, but
  `src/lib/server/services/vendmaster/commission.test.ts` was not updated
```

---

## Category 9: Testing Gaps

- New functions without corresponding tests
- Modified functions with no test updates
- Test file exists but doesn't cover the changed code paths
- Missing edge case tests for error handling paths

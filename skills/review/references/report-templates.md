# Report Templates

## Full Review Report Structure

The review produces a structured report with these sections in order:

### 1. Walkthrough

A narrative paragraph explaining what the changes do and why, written in plain English. This orients the reviewer before they see findings.

```markdown
## Walkthrough

This PR adds rate limiting to the authentication endpoints and fixes a race condition
in the order checkout flow. The rate limiter uses a sliding window algorithm stored in
PostgreSQL, applied to `/api/auth/login` and `/api/auth/reset-password`. The checkout
fix adds optimistic locking via a version column on the orders table.
```

### 2. Changes Table

A concise file-by-file summary:

```markdown
## Changes

| File | Change |
|------|--------|
| `src/lib/server/rate-limiter.ts` | New sliding window rate limiter |
| `src/routes/api/auth/login/+server.ts` | Added rate limiting middleware |
| `src/routes/api/orders/+server.ts` | Added version check for optimistic locking |
| `sql/migrations/020-add-order-version.sql` | Added `version` column to orders |
```

### 3. Findings (Grouped by Severity)

```markdown
## Findings

### Critical (Must Fix)
1. **[SECURITY] SQL injection in `src/routes/api/users/+server.ts:45`**
   - Raw user input in query: `sql.unsafe(\`... ${search} ...\`)`
   - **Fix**: Use tagged template: `` sql`... ${search} ...` ``

### Issues (Should Fix)
2. **[TYPE] `any` type in `src/lib/utils.ts:23`**
   - Defeats TypeScript safety
   - **Fix**: Define proper interface or use `unknown` with type guard

3. **[CROSS-FILE] Missing co-change for `commission.test.ts`**
   - `commission.ts` was modified but tests were not updated

### Suggestions (Consider)
4. **[PERF] Sequential fetches in `src/routes/leads/+page.server.ts:34`**
   - `await getUser()` then `await getLeads()` — could be parallel
   - **Fix**: `const [user, leads] = await Promise.all([getUser(), getLeads()])`

### Nitpicks (Minor)
<details><summary>2 nitpicks</summary>

5. **[STYLE] Inconsistent import order in `src/routes/api/auth/+server.ts`**
   - External imports should come before `$lib/` imports

6. **[STYLE] Unused variable `temp` in `src/lib/server/services/email/send.ts:12`**
</details>
```

### 4. Tech Debt Summary

Only shown when tech debt or duplication is found:

```markdown
## Tech Debt

| Type | Count | Details |
|------|-------|---------|
| Duplicated blocks | 2 | See findings #7, #8 |
| Dead code | 1 | `src/lib/utils/legacy.ts` |
| TODOs without tickets | 3 | Lines 67, 112, 245 |

**Estimated cleanup effort**: Small (< 1 hour)
```

### 5. Passed Checks

Collapsed by default to keep focus on issues:

```markdown
<details><summary>Passed Checks (8)</summary>

- No hardcoded secrets detected
- All form inputs have labels
- Error responses are consistent
- Pagination on all list endpoints
- Rate limiting on auth endpoints
- HTTPS enforced
- Session cookies are httpOnly
- Test files exist for new modules
</details>
```

---

## Commit Message Template (After Fixes)

When committing review fixes, stage specific files (NEVER `git add -A`):

```bash
git add <specific-fixed-files>
git commit -m "fix: address code review findings

Security:
- Fix SQL injection in user query
- Add rate limiting to login endpoint

Type Safety:
- Replace \`any\` with proper interface

Accessibility:
- Add aria-label to menu toggle button"
```

---

## Fix Dispatch Pattern

For approved fixes, launch parallel subagents grouped by file:

```
For each file with approved fixes:
  Task(subagent_type='general-purpose', model='sonnet', prompt=
    "Fix the following issues in {file}:
     {list of fixes with descriptions and solutions}
     Apply fixes following project standards.
     Do NOT make any other changes.
     Do NOT add comments explaining the fix.")
```

After fixes complete:
1. Run `git diff` to verify changes
2. Check for any new issues introduced
3. Present final summary
4. Wait for user to approve commit

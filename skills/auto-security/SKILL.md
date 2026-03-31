---
name: auto-security
description: "Security patterns Claude inconsistently applies: session token hashing, auth information hiding, timing-safe flows, cookie hardening, and file upload validation. Use when implementing auth, sessions, file uploads, or reviewing security. Triggers: security, auth, login, session, cookie, upload, password, hash, rate limit, CSRF, XSS, injection, OWASP, vulnerability, permission, access control, brute force, timing attack."
---

# Security — What Claude Gets Wrong

These are patterns Claude knows but inconsistently applies. This skill exists to enforce them every time.

## Sessions: Hash Tokens Before Storage

**Claude often stores raw session tokens in the database.** If the DB leaks, every session is compromised.

```
ALWAYS: Store SHA-256(token) in DB, send raw token in cookie
NEVER:  Store raw token in DB
```

```typescript
import { createHash, randomBytes } from 'node:crypto';

function generateToken(): string {
  return randomBytes(32).toString('base64url');
}

function hashToken(token: string): Buffer {
  return createHash('sha256').update(token).digest();
}

// INSERT INTO sessions (token_hash, ...) VALUES (${hashToken(token)}, ...)
// SELECT ... WHERE token_hash = ${hashToken(cookieToken)}
```

## Auth: Hide Everything

Claude sometimes returns detailed validation errors on auth endpoints, leaking input shape to attackers.

- **Same generic error** for missing user, wrong password, and invalid input: `"Invalid credentials"`
- **404 not 401/403** for protected endpoints when unauthenticated (hides endpoint existence)
- **Never return field-specific errors** on login (`"email is required"` tells attacker the field name)

## Auth: Timing-Safe Flow

Claude sometimes skips the dummy hash on missing users, creating a timing oracle for user enumeration.

```typescript
const DUMMY_HASH = await bcrypt.hash('timing-safe-dummy', 12); // compute once at startup

// ALWAYS run bcrypt.compare, even when user doesn't exist
const user = await findUser(email);
const hash = user?.password_hash ?? DUMMY_HASH;
const valid = await bcrypt.compare(password, hash);
if (!user || !valid) return error(401, 'Invalid credentials');
```

## Auth: Session Fixation Prevention

Claude sometimes creates new sessions without destroying the old one.

```
On login: DELETE old session (from cookie) THEN INSERT new session
On password change: DELETE all OTHER sessions for user, keep current
```

## Cookie Flags: Always Strict

Claude sometimes defaults to `sameSite: 'lax'`. For auth cookies, always use `'strict'`.

```typescript
cookies.set('session', token, {
  httpOnly: true,     // no JS access
  secure: true,       // HTTPS only (except dev)
  sameSite: 'strict', // NOT 'lax' — strict prevents CSRF
  path: '/',
  maxAge: ttlSeconds,
});
```

## Password Hashing: bcrypt 12+

Claude sometimes uses cost 10. The minimum for 2025+ is **12**.

```typescript
const hash = await bcrypt.hash(password, 12);
```

Alternatives: Argon2id (preferred if available), scrypt (Node built-in).

## File Uploads: Three-Layer Validation

Claude sometimes skips magic byte verification or uses only Content-Type.

```
1. MIME allowlist check (Content-Type header)
2. Magic byte verification (actual file content)
3. Cross-verify: declared MIME must match detected MIME
```

Also: UUID filenames (never use user input in paths), path traversal guard (`resolve()` check), empty file rejection.

## Dangerous Patterns — Flag on Sight

| Pattern | Safe Alternative |
|---------|-----------------|
| `eval(userInput)` | `JSON.parse()` or redesign |
| `child_process.exec(cmd)` | `execFile('cmd', [args])` |
| `.innerHTML = userInput` | `.textContent` or DOMPurify |
| `{@html userInput}` (Svelte) | `{userInput}` or DOMPurify |
| `${{ github.event.*.body }}` in GH Actions `run:` | Use `env:` block |

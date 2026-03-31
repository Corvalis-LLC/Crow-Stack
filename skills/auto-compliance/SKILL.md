---
name: auto-compliance
description: "Privacy and compliance patterns Claude inconsistently applies: GPC header honoring, financial record anonymization vs deletion, deletion confirmation gates, consent proof storage, and regulatory escalation triggers. Use when handling user data, consent, deletion, payments, or privacy features. Triggers: compliance, GDPR, CCPA, privacy, PII, consent, cookie, data retention, right to delete, data export, payment, breach notification, data subject request."
---

# Compliance — What Claude Gets Wrong

These are regulatory patterns Claude knows conceptually but inconsistently implements. This skill enforces the specifics that matter in court.

## CCPA: Honor Global Privacy Control

**Claude often misses GPC.** The `Sec-GPC: 1` header is a legally binding opt-out signal under CCPA. If present, treat it as "Do Not Sell or Share" without requiring any UI interaction.

```typescript
// In hooks.server.ts — check on EVERY request
const gpc = request.headers.get('sec-gpc') === '1';
if (gpc) {
  // Force marketing/advertising consent to denied
  // Do NOT set tracking cookies
  // Honor as equivalent to clicking "Do Not Sell"
}
```

Also required: a visible "Do Not Sell or Share My Personal Information" link on homepage.

## GDPR Deletion: Anonymize, Don't Delete Financial Records

Claude sometimes `DELETE FROM` everything. Financial records (orders, payments, invoices) must be **retained for 7 years** per tax law (GDPR Art. 17(3)(b) exemption).

```
CORRECT: UPDATE orders SET customer_name='REDACTED', user_id=NULL WHERE user_id=$1
WRONG:   DELETE FROM orders WHERE user_id=$1
```

- Strip ALL PII fields (name, email, address, IP, user agent)
- Set `user_id = NULL` (sever the link)
- Stamp `anonymized_at = NOW()`
- Keep transaction amounts, dates, tax IDs intact

## Deletion: Require Explicit Confirmation

Claude sometimes implements deletion without a confirmation gate. Account deletion is irreversible — require proof of intent.

```typescript
// Require explicit confirmation string in request body
const { confirm } = await request.json();
if (confirm !== 'DELETE_MY_ACCOUNT') {
  throw error(400, 'Send { "confirm": "DELETE_MY_ACCOUNT" } to proceed');
}
```

Also: consider a 72-hour cooling-off period (GDPR Art. 12(3) allows up to 1 month).

## Consent: Defaults Must Be Denied

Claude usually gets this right but occasionally pre-enables analytics. Enforce:

- All non-essential consent categories default to `false`
- `necessary` cookies are always `true` and cannot be toggled
- Accept All and Reject All buttons must have **equal visual prominence**
- Scripts MUST NOT load until their consent category is granted

```typescript
const DEFAULT_CONSENT = {
  necessary: true,   // locked
  analytics: false,  // user must opt in
  functional: false, // user must opt in
  marketing: false,  // user must opt in
};
```

## Consent: Store Proof

Every consent action needs an audit trail. Store: timestamp, categories chosen, consent version, IP hash (not raw IP), source (banner/preferences/API). This is your defense in litigation.

## When to Stop and Flag for Legal Review

These trigger mandatory legal consultation — do not implement without legal sign-off:

- **Health data** (HIPAA) — any diagnosis, medication, biometric, or health-related field
- **Minors' data** — under 13 (COPPA), 13-16 in California (CCPA)
- **Biometric data** — fingerprints, face scans, voice prints
- **International transfers** — EU→US data flows require Standard Contractual Clauses
- **Selling/sharing PII with third parties** — triggers CCPA disclosure requirements
- **AI-generated voice calls** — FCC requires prior consent + AI disclosure
- **Financial transactions > $10k** — AML reporting obligations

## Breach Notification Deadlines

If you discover a data breach, the clock starts immediately:

| Jurisdiction | Deadline | Notify |
|-------------|----------|--------|
| GDPR (EU) | **72 hours** | Supervisory authority + affected users if high risk |
| CCPA (California) | "Most expedient time possible" | Attorney General if 500+ residents |
| HIPAA | 60 days | HHS + affected individuals |
| Most US states | 30-60 days | State AG + affected individuals |

## Never Log PII

```
WRONG: logger.info('Login', { email, password, ip })
RIGHT: logger.info('Login', { userId, timestamp })
```

Hash IPs before storing in audit logs. Never log passwords, tokens, or full API keys.

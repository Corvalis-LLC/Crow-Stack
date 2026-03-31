---
name: security-scan
description: "Active security vulnerability scanner. Runs a Node.js script that scans for dangerous patterns, missing auth, hardcoded secrets, plus npm audit and Semgrep (if available)."
disable-model-invocation: true
---

# Security Scan

Run the security scanner against the codebase.

## Step 1: Run the Scanner

Execute this Bash command:

```
node .claude/scripts/security-scan.mjs $ARGUMENTS
```

If `$ARGUMENTS` is empty, the script defaults to scanning `src/`.

The script runs three tiers of analysis:
1. **Custom regex scanner** (always runs) — 10-phase check for code injection, XSS, SQL injection, hardcoded secrets, missing auth, unvalidated input, insecure cookies, info leakage, prototype pollution, open redirects
2. **npm audit** (always available) — dependency vulnerability check
3. **Semgrep** (runs if installed) — deep pattern matching with OWASP rules

## Step 2: Parse the JSON Output

The script outputs a JSON report. Parse it and present the results.

## Step 3: Present the Report

Format the JSON into this report structure:

```
## Security Scan Report

**Scanned:** [filesScanned] files in [scope]
**Date:** [date]

### Tools Used

| Tool | Status |
|------|--------|
| Custom Scanner | [ran] |
| npm audit | [ran/skipped] |
| Semgrep | [ran/skipped] |

### Summary

| Severity | Count |
|----------|-------|
| CRITICAL | [N]   |
| HIGH     | [N]   |
| MEDIUM   | [N]   |
| LOW      | [N]   |
```

If `npmAudit` is present in the report, add:

```
### Dependency Vulnerabilities (npm audit)

| Severity | Count |
|----------|-------|
| Critical | [N]   |
| High     | [N]   |
| Moderate | [N]   |
| Low      | [N]   |

[If total > 0]: Run `npm audit` for full details and `npm audit fix` to auto-fix.
```

### Findings

For each finding in the `findings` array, grouped by severity:

```
### [SEVERITY] Issues

#### [S-NNN] [title]
- **File:** [file]:[line]
- **Phase:** [phase]
- **Code:** `[code]`
- **Risk:** [risk]
- **Fix:** [fix]
```

Number findings sequentially within severity: C-001, C-002... for CRITICAL, H-001... for HIGH, M-001... for MEDIUM, L-001... for LOW.

### Passed Checks

List each item from the `passed` array:

```
### Passed Checks
- [message for each passed phase]
```

## Step 4: Recommendations

After presenting the report:

1. If CRITICAL findings exist: **"CRITICAL issues found. These should be fixed before any deployment."**
2. If Semgrep was skipped: **"For deeper analysis, install Semgrep: `npm i -g @semgrep/cli` — then re-run this scan."**
3. If npm audit found vulnerabilities: **"Run `npm audit fix` to auto-patch dependency vulnerabilities."**
4. If all phases passed: **"Clean scan. No security issues detected."**

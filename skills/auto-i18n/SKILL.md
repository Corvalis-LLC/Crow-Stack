---
name: auto-i18n
description: "Internationalization discipline: ICU pluralization, locale-aware formatting, RTL support, named placeholders, translation key organization, and locale fallback chains. Corrects manual pluralization, concatenated strings, hardcoded number/date formats, and missing bidi isolation. Use when adding user-facing strings, formatting numbers/dates/currencies, supporting multiple locales, or handling RTL layouts. Triggers: i18n, internationalization, locale, translation, plural, pluralization, ICU, MessageFormat, Fluent, RTL, bidi, right-to-left, Intl, NumberFormat, DateTimeFormat, gettext, t(), formatMessage, locale detection, language, l10n, localization."
---

# auto-i18n — Internationalization Discipline

You default to English-only string handling: manual if/else pluralization, concatenated translated fragments, hardcoded date/number formats, and zero RTL awareness. These patterns break the moment a second locale appears.

## Rule 1: ICU/Fluent Pluralization — Never Manual If/Else

Your pattern:
```ts
const label = count === 1 ? t('one_item') : t('many_items');
```

This breaks for languages with 2, 3, or 6 plural forms (Arabic has 6). Always use ICU MessageFormat or Fluent plural selectors.

| Language | Plural Forms | Example Boundaries |
|----------|-------------|-------------------|
| English | 2 (one, other) | 1 / everything else |
| French | 2 (one, other) | 0-1 / 2+ |
| Polish | 3 (one, few, many) | 1 / 2-4 / 5-21 |
| Arabic | 6 (zero, one, two, few, many, other) | 0 / 1 / 2 / 3-10 / 11-99 / 100+ |

**Correct — ICU MessageFormat:**
```
{count, plural,
  =0 {No items}
  one {{count} item}
  other {{count} items}
}
```

**Correct — Fluent:**
```ftl
items-count = { $count ->
    [0] No items
    [one] {$count} item
   *[other] {$count} items
}
```

**Correct — Rust (fluent-rs or icu4x):**
```rust
// fluent-rs
let mut args = FluentArgs::new();
args.set("count", item_count);
bundle.format_pattern(pattern, Some(&args), &mut errors)
```

Never write plural logic in application code. The i18n library handles CLDR plural rules per locale.

## Rule 2: Locale-Aware Formatting — Never Hardcode Formats

Your pattern:
```ts
const display = `$${price.toFixed(2)}`;
const date = `${d.getMonth()+1}/${d.getDate()}/${d.getFullYear()}`;
```

This produces `$1,234.56` for en-US but should be `1.234,56 €` for de-DE. Use `Intl` APIs (JS/TS) or `icu4x`/`num-format` (Rust).

| Data Type | JS/TS | Rust |
|-----------|-------|------|
| Currency | `Intl.NumberFormat(locale, {style:'currency', currency})` | `icu4x` FixedDecimalFormatter + CurrencyFormatter |
| Number | `Intl.NumberFormat(locale)` | `icu4x` FixedDecimalFormatter |
| Date | `Intl.DateTimeFormat(locale, options)` | `icu4x` DateTimeFormatter or `chrono` + format per locale |
| Relative time | `Intl.RelativeTimeFormat(locale)` | manual or `icu4x` |
| List | `Intl.ListFormat(locale)` | manual join with locale separator |
| Percent | `Intl.NumberFormat(locale, {style:'percent'})` | `icu4x` |

**Correct:**
```ts
new Intl.NumberFormat(locale, {
  style: 'currency',
  currency: userCurrency,
}).format(price)
```

Never assume decimal separator is `.`, thousands separator is `,`, or currency symbol precedes the number.

## Rule 3: No String Concatenation for Translations

Your pattern:
```ts
t('welcome') + ' ' + userName + ' ' + t('to_dashboard')
```

Word order differs across languages. Japanese: `{name}さん、ダッシュボードへようこそ`. Arabic reads right-to-left. Concatenation makes reordering impossible.

**Correct — named placeholders:**
```ts
t('welcome_message', { name: userName })
// en: "Welcome {name} to the dashboard"
// ja: "{name}さん、ダッシュボードへようこそ"
```

**Correct — Rust Fluent:**
```ftl
welcome-message = Welcome { $name } to the dashboard
```

Rules:
- Every dynamic value is a **named placeholder**, never positional
- Translators can reorder placeholders freely
- Never split a sentence across multiple translation keys
- Never embed HTML tags via concatenation — use rich text placeholders (`<bold>{name}</bold>`)

## Rule 4: RTL Support — Logical Properties + Dir Attribute

You produce `margin-left`, `text-align: left`, `padding-right` which break in RTL locales (Arabic, Hebrew, Farsi, Urdu).

| Physical (breaks RTL) | Logical (works both) |
|-----------------------|---------------------|
| `margin-left` | `margin-inline-start` |
| `padding-right` | `padding-inline-end` |
| `text-align: left` | `text-align: start` |
| `left: 0` | `inset-inline-start: 0` |
| `border-right` | `border-inline-end` |
| `float: left` | `float: inline-start` |

**Additional RTL requirements:**
- Set `<html lang={locale} dir={dir}>` — derive `dir` from locale, don't hardcode
- Icons with directional meaning (arrows, progress) must flip: `transform: scaleX(-1)` when `[dir="rtl"]`
- Bidi isolation for user-generated content: `<bdi>` element or `unicode-bidi: isolate` to prevent text direction leaking
- **React Native:** Use `I18nManager.isRTL` and `writingDirection` style property; avoid absolute `left`/`right` positioning

## Rule 5: Locale Detection and Fallback Chains

Your pattern:
```ts
const locale = navigator.language || 'en';
```

This gives `en-US` but your translations might only have `en`. You need a negotiation chain.

**Correct fallback order:**
1. User's explicit preference (DB/cookie/setting)
2. `Accept-Language` header (server) or `navigator.languages` (client) — note: plural, returns array
3. Negotiate against available locales
4. Fall back to default locale

**Correct — JS:**
```ts
// Use Intl.LocaleMatcher or a library like @formatjs/intl-localematcher
const negotiated = match(
  requestedLocales,   // ['fr-CA', 'fr', 'en']
  availableLocales,   // ['en', 'fr', 'de']
  defaultLocale       // 'en'
);
```

**Correct — Rust:**
```rust
// fluent-langneg crate
let negotiated = negotiate_languages(
    &requested,
    &available,
    Some(&default_locale),
    NegotiationStrategy::Filtering,
);
```

Never assume a 1:1 match between requested and available locales. `pt-BR` should fall back to `pt`, not to `en`.

## Rule 6: Translation Key Organization

Your pattern: flat keys like `t('button')`, `t('title')`, `t('error')` — collide across pages and components.

**Correct — namespaced keys:**
```
# Structured by feature/page
dashboard.header.title = Dashboard
dashboard.header.subtitle = Overview of your account
settings.profile.name_label = Full Name
settings.profile.save_button = Save Changes
errors.network.timeout = Request timed out. Please try again.
```

Rules:
- Keys are **namespaced by feature**, not by page layout position
- Keys describe **semantic meaning**, not visual placement (`save_button` not `bottom_right_button`)
- Shared strings (Save, Cancel, Delete) go in a `common.*` namespace
- Never reuse a key across unrelated contexts — "Save" on a form vs "Save" on a game screen may translate differently
- Keep translation files in a dedicated directory (`locales/`, `messages/`, `i18n/`), never inline in components

## Rule 7: Externalize All User-Facing Strings

Your pattern: hardcoded strings mixed into components.
```svelte
<button>Save Changes</button>
<p>No results found</p>
```

Every user-visible string must go through the translation function, even if you only support one language today. Retrofitting i18n is 10x harder than designing for it.

**Correct:**
```svelte
<button>{$t('common.save')}</button>
<p>{$t('search.no_results')}</p>
```

What to externalize:
- All UI labels, buttons, headings, placeholders
- Error messages shown to users
- Notification text, toast messages
- Email/SMS templates
- Accessibility labels (`aria-label`, `alt` text)

What NOT to externalize:
- Log messages (always English for grep-ability)
- Internal error codes
- API field names
- Developer-facing config keys

## Anti-Pattern Summary

| # | Anti-Pattern | Correct Pattern |
|---|-------------|----------------|
| 1 | Manual if/else pluralization | ICU MessageFormat / Fluent plural selectors |
| 2 | `toFixed()`, manual date format | `Intl.NumberFormat`, `Intl.DateTimeFormat`, icu4x |
| 3 | String concatenation for sentences | Named placeholders in translation strings |
| 4 | Physical CSS properties (`margin-left`) | Logical properties (`margin-inline-start`) |
| 5 | `navigator.language \|\| 'en'` | Locale negotiation with fallback chain |
| 6 | Flat keys (`t('title')`) | Namespaced keys (`dashboard.header.title`) |
| 7 | Hardcoded strings in components | All user-facing strings through `t()` |

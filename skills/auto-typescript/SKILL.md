---
name: auto-typescript
description: "TypeScript type safety discipline: eliminates as any/unknown casts, enforces proper narrowing, branded types, Zod pitfalls, strict config, and type-level patterns. Use when writing TypeScript involving domain types, Zod validation, type assertions, or complex generics. Triggers: typescript, type, zod, branded, .ts, generic, type error, as any, unknown, type assertion, cast."
---

# TypeScript — Type Safety Discipline

Claude writes TypeScript that compiles but lies about types. Your most common failure: reaching for `as any` or `as unknown as X` when you can't immediately figure out the correct type. This skill fixes that and covers the patterns you don't reach for unprompted.

## The Cardinal Rule: No Lying to the Compiler

Every type assertion (`as X`) is a claim: "I know better than the compiler." Most of the time, you don't — you're just silencing an error you haven't investigated.

**Before writing any `as` assertion, ask:** "Can I make the compiler agree with me instead?"

| You reach for... | Do this instead |
|---|---|
| `as any` | Fix the actual type. This is never acceptable in production code. |
| `as unknown as X` | Find the actual relationship between the types. Double-cast means your types are wrong. |
| `as X` on a function return | Add a return type annotation and fix the implementation. |
| `as X` on an API response | Parse with Zod or write a type guard. |
| `as X` on a DOM element | Use a type guard: `if (el instanceof HTMLInputElement)` |

## Type Assertions You Must Eliminate

### `as any` — Never Acceptable

```typescript
// You write:
const result = fetchData() as any;
processResult(result.name); // No type safety at all

// Senior writes:
interface FetchResult { name: string; value: number; }
const result: FetchResult = await fetchData();
processResult(result.name); // Compiler verifies .name exists
```

If you can't type something, that's a signal to investigate — not to give up. The ONE exception: test mocks where you're deliberately providing a partial implementation (and even then, prefer `Partial<T>` or a proper mock builder).

### `as unknown as X` — Your Types Are Wrong

The double-cast is always a design smell. It means the types don't model reality.

```typescript
// You write:
const user = row as unknown as User; // DB row doesn't match User? Fix the type.

// Senior writes — option A: type the query result
interface UserRow { id: string; name: string; created_at: string; }
const row: UserRow = rows[0]; // matches what the DB actually returns
const user: User = { id: row.id, name: row.name, createdAt: new Date(row.created_at) };

// Senior writes — option B: parse with Zod
const user = userSchema.parse(row); // validates AND types in one step
```

### `@ts-ignore` / `@ts-expect-error` — Last Resort Only

These comments hide errors instead of fixing them. Only acceptable when:
1. A library has incorrect type definitions (with a comment linking to the upstream issue)
2. You've confirmed the code is correct and the compiler can't prove it

Always prefer `@ts-expect-error` over `@ts-ignore` — it will alert you when the underlying issue is fixed.

## Narrowing Over Assertion

Type guards give you runtime safety AND compile-time narrowing. Assertions give you neither.

```typescript
// You write — assertion, no runtime check:
function processEvent(event: unknown) {
  const typed = event as ClickEvent;
  handleClick(typed.x, typed.y); // crashes if event isn't a ClickEvent
}

// Senior writes — narrowing, runtime safe:
function isClickEvent(event: unknown): event is ClickEvent {
  return (
    typeof event === 'object' && event !== null &&
    'x' in event && 'y' in event &&
    typeof (event as Record<string, unknown>).x === 'number'
  );
}

function processEvent(event: unknown) {
  if (isClickEvent(event)) {
    handleClick(event.x, event.y); // compiler knows these exist
  }
}
```

### Discriminated Unions — The Correct Way to Narrow

When a value can be multiple shapes, use a discriminant field:

```typescript
// You write — checking with assertions:
type Result = SuccessResult | ErrorResult;
function handle(result: Result) {
  if ((result as SuccessResult).data) { ... } // WRONG — assertion doesn't narrow
}

// Senior writes — discriminant narrows automatically:
type Result =
  | { status: 'success'; data: Payload }
  | { status: 'error'; message: string };

function handle(result: Result) {
  if (result.status === 'success') {
    result.data; // TypeScript knows this is Payload
  }
}
```

### `satisfies` Over `as` for Validation

When you want to check that a value matches a type without losing literal info:

```typescript
// You write — widens the type:
const config = { port: 3000, host: 'localhost' } as Config;
// config.port is number, not 3000

// Senior writes — validates without widening:
const config = { port: 3000, host: 'localhost' } satisfies Config;
// config.port is 3000 (literal), AND Config shape is verified
```

## Return Types — Don't Let Inference Lie

Implicit return types can silently widen or change when you edit the function body. Always annotate return types on:
- **Exported functions** — the public contract should be explicit
- **Functions returning complex types** — unions, generics, conditional types
- **Async functions** — `Promise<X>` is clearer than inference

```typescript
// You write — return type changes silently if you edit the body:
export function getUser(id: string) {
  return db.query(`SELECT * FROM users WHERE id = ${id}`);
}

// Senior writes — contract is locked:
export function getUser(id: string): Promise<UserRow | undefined> {
  return db.query(`SELECT * FROM users WHERE id = ${id}`);
}
```

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| `as any` anywhere | `const x = foo as any` | Type it correctly. No exceptions. |
| Double cast | `x as unknown as Y` | Fix the source type or parse the data |
| Assertion for narrowing | `(x as Foo).bar` | Type guard: `if ('bar' in x)` or `isFoo(x)` |
| Untyped destructuring | `const { data } = await res.json()` | `const { data }: { data: UserRow[] } = await res.json()` or parse with Zod |
| Implicit `any` in callbacks | `arr.map(item => ...)` where item is any | `arr.map((item: SpecificType) => ...)` |
| Optional chaining as type narrowing | `obj?.prop?.method()` hides null bugs | Narrow explicitly, handle the null case |
| `!` non-null assertion | `user!.name` | Guard: `if (!user) throw ...` or `if (user) { user.name }` |
| Bare `object` or `{}` types | `function process(data: object)` | Use a specific interface or `Record<string, unknown>` |
| `Function` type | `callback: Function` | `callback: (arg: string) => void` — type the signature |

## The `unknown` Boundary Pattern

External data (API responses, user input, DB results, JSON.parse) enters your system as `unknown`. Handle it at the boundary, then the interior is type-safe:

```typescript
// At the boundary — validate and parse:
const body: unknown = await request.json();
const parsed = createUserSchema.parse(body); // Zod validates, returns typed
// parsed is now CreateUser — no assertions needed anywhere downstream

// Interior code — fully typed, no assertions:
function createUser(input: CreateUser): Promise<User> {
  // input is guaranteed valid by the boundary
}
```

Never skip the boundary validation. If you're tempted to write `as SomeType` on external data, add Zod parsing instead.

## Branded Types for Domain Safety

When two values share a primitive type but represent different concepts, brand them:

```typescript
type Brand<T, B extends string> = T & { readonly __brand: B };

type UserId = Brand<string, 'UserId'>;
type PostId = Brand<string, 'PostId'>;
type Cents = Brand<number, 'Cents'>;

function getPost(id: PostId): Post { ... }
getPost(userId); // Compile error — caught at build time
```

Reach for branded types when multiple string/number params could be accidentally swapped, or when a value has been validated and you want the type system to remember that.

## Zod v4 Pitfalls

```typescript
// z.record() requires TWO arguments (key schema + value schema)
z.record(z.string())              // WRONG — Zod v4 error
z.record(z.string(), z.unknown()) // RIGHT

// Error details are in .issues, not .errors
parsed.error.errors[0].message  // WRONG — undefined
parsed.error.issues[0].message  // RIGHT
```

## Strict tsconfig — The Two You're Missing

```json
{
  "noUncheckedIndexedAccess": true,
  "exactOptionalPropertyTypes": true
}
```

`noUncheckedIndexedAccess` makes `arr[0]` return `T | undefined`. `exactOptionalPropertyTypes` distinguishes "property is missing" from "property is undefined."

## NoInfer for Controlled Inference

```typescript
// Without NoInfer: defaultValue widens the type
function getOrDefault<T>(values: T[], defaultValue: T): T { ... }
getOrDefault([1, 2, 3], 'oops'); // T = string | number — no error!

// With NoInfer: only values determines T
function getOrDefault<T>(values: T[], defaultValue: NoInfer<T>): T { ... }
getOrDefault([1, 2, 3], 'oops'); // Error: string not assignable to number
```

## Prefer `as const satisfies` Combo

When you need both literal preservation AND type checking:

```typescript
const ROUTES = {
  home: '/',
  about: '/about',
  dashboard: '/dashboard',
} as const satisfies Record<string, `/${string}`>;

// ROUTES.home is literally '/' (not string)
// AND the compiler verified every value starts with '/'
```

## Datetime Handling — Never Use Raw `Date`

`Date` has no timezone support, mutates in place, and silently produces wrong results across timezones. Use `Temporal` (via `@js-temporal/polyfill`), Luxon, or `date-fns-tz` for timezone-aware work. Raw `Date` is only acceptable for UTC-only timestamps with no timezone conversion.

Key rules:
- **Never accept a timezone parameter and then ignore it** — this is your most common datetime bug
- **Add calendar days, not milliseconds** — `date.add({ days: 7 })` handles DST; adding 604800000ms doesn't
- **Use `.toInstant()` for server storage** — store UTC instants, convert to zoned for display
- **`Intl.DateTimeFormat` for display** — never manual string formatting

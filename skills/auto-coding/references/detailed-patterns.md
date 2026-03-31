# Detailed Coding Patterns

Extended patterns referenced by the auto-coding SKILL.md. Load on demand for deeper guidance.

---

## File Organization

### Internal File Structure (top to bottom)
1. Module-level documentation (if needed)
2. Imports/use statements (grouped and ordered)
3. Constants and configuration
4. Type definitions (interfaces, types, structs, enums)
5. Main exports (functions, classes, components)
6. Helper/utility functions (private to the module)

### Module Organization
- **One concept per file.** A file named `user-service` should only contain user-related service logic.
- **No god files.** If a utils file grows past 200 lines, split by domain: `date-utils`, `string-utils`, `currency-utils`.
- **Barrel/re-export files only at package boundaries.** Never internally — they cause circular dependencies, slow builds, and bundle bloat.

---

## Commenting Deep Dive

### The Cardinal Rule: "Why, Not What"
Comments explain WHY code exists or WHY a particular approach was chosen — never WHAT the code does.

### When to Comment
1. **Non-obvious business logic**: `// Commission caps at 15% per the 2024 vendor agreement`
2. **Workarounds**: `// Workaround for upstream bug #12345 where...`
3. **Performance decisions**: `// Using HashMap for O(1) lookup on large datasets`
4. **Regex explanations**: Always comment non-trivial regex
5. **TODO with tracking**: `// TODO(#1234): Migrate to cursor pagination`

### When NOT to Comment
1. **Restating the code**: If the comment says the same thing as the code, delete it
2. **Obvious code**: `let name = user.name; // get user's name`
3. **Commented-out code**: Delete it. Version control exists.
4. **Journal comments**: `// 2024-01-15: Changed X to Y` — use git blame
5. **Closing brace comments**: `} // end if` — function is too long if needed

---

## Future-Proofing Patterns

### Depend on Abstractions, Not Implementations
Code against interfaces/traits/protocols, not concrete implementations. Pass dependencies as parameters rather than importing globals directly.

### Design for Deletion, Not Extension
Structure code so any module can be deleted without cascading changes. Low coupling means you can rip out features cleanly.

### Keep Business Logic Framework-Agnostic
Business logic (services, validation, calculations) should never import from framework-specific modules. Framework code calls into business logic, never the reverse.

### Prefer Data Over Code
When you see repeated conditionals mapping values, replace with a lookup structure. Add new entries by adding data, not code.

### Favor Composition Over Inheritance
Build features by composing small, focused functions and components rather than extending base classes. Inheritance hierarchies become rigid; composition allows mixing capabilities freely.

### Thin Adapters at Boundaries
Keep framework-specific code (route handlers, component wrappers) as thin as possible. They should only translate between the framework's world and your business logic. When frameworks change, you only rewrite the thin adapter layer.

---

## Universal Anti-Patterns

| Anti-Pattern | Why It's Dangerous | Use Instead |
|---|---|---|
| Magic numbers/strings | Unexplained `3` or `'admin'` in logic | Named constants |
| Deep nesting (>3 levels) | Arrow-shaped code hides error paths | Early returns, extract functions |
| God functions ("validates AND saves AND notifies") | Untestable, unreusable | Split by responsibility |
| Commented-out code | Rots, confuses readers | Delete it (use git) |
| Empty catch/except blocks | Silent failures are hardest bugs to diagnose | Log, handle, or re-throw |
| Mutating function arguments | Creates invisible side effects | Return new values |
| Premature optimization | Sacrifices readability for theoretical performance | Profile first, optimize second |
| Boolean parameters | `create_user(name, True, False)` — what do they mean? | Options object/struct or separate functions |

## Universal Best Practices

| Practice | Why | Example |
|---|---|---|
| Early returns / guard clauses | Flattens nesting, makes error cases explicit | `if not user: return` at top |
| Single responsibility | Every function/module has one reason to change | Split functions with "and" in description |
| Explicit over implicit | New team members don't know implicit rules | Named arguments, explicit returns |
| Consistent error handling | Consumers shouldn't guess error shapes | One error pattern everywhere |
| Immutable by default | Eliminates mutation bugs | Only use mutable when reassignment needed |
| Fail fast | Bad data caught close to source | Validate at system boundaries |
| Rule of Three for DRY | Premature abstraction worse than duplication | Extract at 3+ occurrences |

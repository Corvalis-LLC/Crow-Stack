---
name: auto-state-machines
description: "State machine discipline: explicit state enums, transition validation, per-state data, audit trails, and impossible state elimination. Corrects boolean flag clusters, stringly-typed status fields, unconstrained transitions, and states that carry data they shouldn't have. Use when modeling workflows, order lifecycles, job statuses, or any entity with distinct phases. Triggers: state machine, state, status, transition, workflow, lifecycle, enum state, order status, job status, phase, stage, step, finite state, FSM, state pattern, boolean flags, status field, state transition."
---

# State Machines — What Claude Gets Wrong

You model states as a `status: String` field and transitions as `status = "next_status"` assignments scattered across the codebase. In production: invalid transitions happen silently, states carry data that doesn't apply to them, boolean flag combinations create impossible states, and nobody can reconstruct how an entity reached its current state.

## The Five Rules

1. **States are enums, not strings** — the compiler enforces valid states
2. **Transitions are validated** — not every state can reach every other state
3. **Per-state data** — each state carries only the data relevant to it
4. **Audit trail** — record every transition with timestamp and reason
5. **No boolean flag clusters** — `(is_paid, is_shipped, is_cancelled)` is a state machine in denial

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| String status | `status: String` | `status: OrderStatus` (enum) |
| Any-to-any transition | `order.status = "shipped"` anywhere | `order.transition(Event::Ship)` with validation |
| Boolean flag cluster | `is_active && !is_suspended && is_verified` | Single enum: `Active`, `Suspended`, `Unverified` |
| Shared data bag | All states access `shipped_at` even before shipping | Per-state data: `Shipped { shipped_at, tracking }` |
| No transition history | Current status only, can't debug how it got there | Append-only transition log |
| Implicit transitions | Status changes buried in business logic | Centralized transition function |

## Explicit State Enums

```rust
// WRONG: stringly typed, any value is "valid"
struct Order {
    status: String,          // "pending"? "PENDING"? "pnding"?
    paid_at: Option<DateTime<Utc>>,    // None when pending, but also None if payment failed?
    shipped_at: Option<DateTime<Utc>>, // Meaningless in most states
    cancelled_reason: Option<String>,  // Only matters when cancelled
}

// RIGHT: each state carries only its relevant data
enum OrderStatus {
    Draft,
    Pending { created_at: DateTime<Utc> },
    Paid { paid_at: DateTime<Utc>, amount: Decimal },
    Shipped { paid_at: DateTime<Utc>, shipped_at: DateTime<Utc>, tracking: String },
    Delivered { delivered_at: DateTime<Utc> },
    Cancelled { cancelled_at: DateTime<Utc>, reason: String },
}
```

```typescript
// WRONG: boolean flag explosion — 2^3 = 8 combinations, most are invalid
interface User {
    isActive: boolean;
    isVerified: boolean;
    isSuspended: boolean;
    // What does isActive && isSuspended mean?
}

// RIGHT: discriminated union — only valid states exist
type UserStatus =
    | { status: "unverified"; registeredAt: Date }
    | { status: "active"; verifiedAt: Date }
    | { status: "suspended"; suspendedAt: Date; reason: string }
    | { status: "deactivated"; deactivatedAt: Date };
```

## Transition Validation

```rust
// WRONG: any code can set any status
order.status = "shipped"; // Can you ship a cancelled order? This code says yes.

// RIGHT: transitions are explicit and validated
impl OrderStatus {
    fn transition(self, event: OrderEvent) -> Result<OrderStatus, InvalidTransition> {
        match (self, event) {
            (OrderStatus::Draft, OrderEvent::Submit) =>
                Ok(OrderStatus::Pending { created_at: Utc::now() }),
            (OrderStatus::Pending { .. }, OrderEvent::Pay { amount }) =>
                Ok(OrderStatus::Paid { paid_at: Utc::now(), amount }),
            (OrderStatus::Paid { paid_at, .. }, OrderEvent::Ship { tracking }) =>
                Ok(OrderStatus::Shipped { paid_at, shipped_at: Utc::now(), tracking }),
            // Cancellation allowed from multiple states
            (OrderStatus::Pending { .. } | OrderStatus::Paid { .. }, OrderEvent::Cancel { reason }) =>
                Ok(OrderStatus::Cancelled { cancelled_at: Utc::now(), reason }),
            (from, event) =>
                Err(InvalidTransition { from, event }),
        }
    }
}
```

```typescript
// Transition map — declarative, auditable
const VALID_TRANSITIONS: Record<string, string[]> = {
    draft:     ["pending"],
    pending:   ["paid", "cancelled"],
    paid:      ["shipped", "cancelled"],
    shipped:   ["delivered"],
    delivered: [],
    cancelled: [],
};

function transition(current: string, next: string): void {
    const allowed = VALID_TRANSITIONS[current];
    if (!allowed?.includes(next)) {
        throw new InvalidTransitionError(current, next);
    }
}
```

## Audit Trail

```rust
// Every transition gets logged — reconstruct history from the log
#[derive(Serialize)]
struct Transition {
    from: String,
    to: String,
    event: String,
    timestamp: DateTime<Utc>,
    actor: String,       // Who/what triggered it
    reason: Option<String>,
}

async fn apply_transition(
    pool: &PgPool,
    order_id: Uuid,
    current: OrderStatus,
    event: OrderEvent,
    actor: &str,
) -> Result<OrderStatus> {
    let next = current.transition(event.clone())?;

    // Record transition
    sqlx::query(
        "INSERT INTO order_transitions (order_id, from_status, to_status, event, actor, created_at)
         VALUES ($1, $2, $3, $4, $5, NOW())"
    )
    .bind(order_id)
    .bind(current.name())
    .bind(next.name())
    .bind(event.name())
    .bind(actor)
    .execute(pool).await?;

    // Update current status
    sqlx::query("UPDATE orders SET status = $1 WHERE id = $2")
        .bind(next.name())
        .bind(order_id)
        .execute(pool).await?;

    Ok(next)
}
```

## Database Representation

```sql
-- Status column stores the enum variant name
-- Per-state data lives in the row (nullable columns) or a JSONB column
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'draft'
        CHECK (status IN ('draft', 'pending', 'paid', 'shipped', 'delivered', 'cancelled')),
    status_data JSONB,  -- Per-state fields like tracking, reason
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Transition log — append-only, never updated or deleted
CREATE TABLE order_transitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id),
    from_status TEXT NOT NULL,
    to_status TEXT NOT NULL,
    event TEXT NOT NULL,
    actor TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_order_transitions_order ON order_transitions(order_id, created_at);
```

## Eliminating Impossible States

The goal: if a state combination can't happen in the real world, make it unrepresentable in your type system.

```rust
// WRONG: Option fields create impossible states
struct Connection {
    state: ConnectionState,
    socket: Option<TcpStream>,    // None when disconnected, Some when connected... but enforced where?
    error: Option<String>,        // Only meaningful in Error state
    retry_count: u32,             // Only meaningful when retrying
}

// RIGHT: per-state data eliminates impossible combinations
enum Connection {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { socket: TcpStream },
    Failed { error: String, attempts: u32 },
}
// Connected ALWAYS has a socket. Failed ALWAYS has an error. No ambiguity.
```

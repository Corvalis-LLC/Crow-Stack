---
name: auto-database
description: "Database query discipline: cursor pagination over OFFSET, index awareness, bulk operations over ORM loops, round-trip minimization, and the specific query anti-patterns Claude defaults to. Corrects N+1 queries, SELECT *, offset pagination, row-by-row updates, and missing index documentation. Use when writing database queries, designing schemas, or optimizing data access. Triggers: database, query, SQL, SELECT, INSERT, UPDATE, DELETE, JOIN, pagination, OFFSET, cursor, index, N+1, ORM, sqlx, prisma, sqlalchemy, transaction, bulk, batch, migration."
---

# Database Queries — What Claude Gets Wrong

You write queries that work at demo scale and fall over at production scale. OFFSET pagination, N+1 loops, ORM-as-a-for-loop, and zero index awareness.

## The Five Rules

1. **Cursor/keyset pagination** — never OFFSET for user-facing lists
2. **Name your columns** — never SELECT *
3. **Document required indexes** — in comments near the query
4. **Bulk operations** — never row-by-row in a loop
5. **Minimize round-trips** — combine queries where possible

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| OFFSET pagination | `LIMIT 20 OFFSET 40` | `WHERE (created_at, id) < ($1, $2) LIMIT 20` |
| SELECT * | `SELECT * FROM users` | `SELECT id, name, email FROM users` |
| N+1 in loop | `for user in users: fetch_orders(user.id)` | Single JOIN or `WHERE user_id = ANY($1)` |
| ORM row-by-row update | `for order in orders: order.status = "archived"` | `UPDATE orders SET status = 'archived' WHERE ...` |
| No index comment | Query on `(status, created_at)` with no index note | `-- Requires: CREATE INDEX idx_orders_status_created ON orders (status, created_at)` |
| Multiple round-trips | 4 separate COUNT queries | Combine: `SELECT COUNT(*), COUNT(*) FILTER (WHERE ...)` |
| Unbounded list | `SELECT ... FROM reports` with no LIMIT | Always LIMIT, even internal queries |
| Load-all-then-filter | `query.fetch_all().filter(|r| r.active)` | `WHERE active = true` in SQL |
| Missing transaction | Two related writes without transaction | `BEGIN; ... COMMIT;` or `pool.begin()` |

## Cursor Pagination Pattern

```rust
// You write (OFFSET — O(n) skip, unstable under inserts):
sqlx::query_as("SELECT id, name FROM projects ORDER BY created_at DESC LIMIT $1 OFFSET $2")
    .bind(limit)
    .bind(page * limit)

// Senior writes (keyset — O(1) seek, stable):
sqlx::query_as(
    "SELECT id, name, created_at FROM projects
     WHERE ($1::timestamptz IS NULL OR (created_at, id) < ($1, $2))
     ORDER BY created_at DESC, id DESC
     LIMIT $3"
)
    .bind(cursor_created_at)
    .bind(cursor_id)
    .bind(limit + 1)  // fetch one extra to detect has_more
```

Return `{ data, pagination: { next_cursor, has_more } }`. Client passes `cursor` on next request.

## Bulk Operations

```python
# You write — N UPDATE statements:
for order in old_orders:
    order.status = "archived"
    session.add(order)
session.commit()

# Senior writes — 1 UPDATE statement:
session.execute(
    update(Order)
    .where(Order.created_at < cutoff)
    .values(status="archived", archived_at=now)
)
session.commit()
```

```typescript
// You write — N queries:
for (const id of userIds) {
  await db.query("UPDATE users SET active = false WHERE id = $1", [id]);
}

// Senior writes — 1 query:
await db.query("UPDATE users SET active = false WHERE id = ANY($1)", [userIds]);
```

## Round-Trip Minimization

```typescript
// You write — 4 round-trips:
const total = await db.count("users");
const active = await db.count("users", { where: { active: true } });
const revenue = await db.sum("orders", "total");
const newToday = await db.count("users", { where: { createdAt: { gte: today } } });

// Senior writes — 1 round-trip:
const [stats] = await db.query(`
  SELECT
    COUNT(*) AS total_users,
    COUNT(*) FILTER (WHERE active) AS active_users,
    (SELECT COALESCE(SUM(total), 0) FROM orders WHERE status = 'completed') AS revenue,
    COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE) AS new_today
  FROM users
`);
```

## Index Documentation

Every query that filters or sorts should document its index requirement:

```rust
// Requires index: CREATE INDEX idx_reports_sentinel_created
//   ON sentinel_reports (sentinel, created_at DESC);
let reports = sqlx::query_as(
    "SELECT id, sentinel, value, created_at
     FROM sentinel_reports
     WHERE sentinel = $1
     ORDER BY created_at DESC
     LIMIT $2"
)
```

This is the single most impactful comment you can write. A query without an index note is a time bomb.

## Transaction Boundaries

Related writes belong in a transaction:

```rust
// You write — partial failure possible:
insert_order(pool, &order).await?;
update_inventory(pool, &items).await?;  // If this fails, order exists without inventory update

// Senior writes:
let mut tx = pool.begin().await?;
insert_order(&mut tx, &order).await?;
update_inventory(&mut tx, &items).await?;
tx.commit().await?;  // All or nothing
```

Rule: if two writes must be consistent, they must be in a transaction.

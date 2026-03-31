---
name: auto-api-design
description: "REST API design discipline: consistent response envelopes, proper HTTP status codes, cursor-based pagination, input validation, structured errors, and DTO separation. Corrects bare returns, naked status codes, offset pagination, missing validation, and leaking internal models. Use when designing or implementing API endpoints. Triggers: api, endpoint, route, handler, REST, response, request, pagination, cursor, status code, 201, 204, 404, 422, validation, error response, DTO, envelope."
---

# API Design — What Claude Gets Wrong

You write endpoints that return bare data, use inconsistent error shapes, skip pagination on lists, and leak database models to clients. Each endpoint looks different. Clients need N parsers for N endpoints.

## The Six Rules

1. **Consistent response envelope** — every endpoint, same shape
2. **Proper HTTP status codes** — not just 200 and 500
3. **Cursor-based pagination** — on every list endpoint
4. **Input validation** — beyond deserialization
5. **Structured error responses** — with codes, not just messages
6. **DTO separation** — API response != database row

## Response Envelope

Every successful response:
```json
{
  "data": { ... },
  "pagination": { "next_cursor": "abc", "has_more": true }
}
```

Every error response:
```json
{
  "error": { "code": "not_found", "message": "Project abc not found" }
}
```

`pagination` is present only on list endpoints. This is the ONLY response shape your API uses.

## HTTP Status Codes

| Operation | Success | Common Errors |
|-----------|---------|--------------|
| GET (single) | 200 | 404 |
| GET (list) | 200 | (never 404 — empty list is 200) |
| POST (create) | 201 | 409 (conflict), 422 (validation) |
| PUT/PATCH (update) | 200 | 404, 422 |
| DELETE | 204 (no body) | 404 |
| Any | — | 400 (malformed), 401, 403, 500 |

Claude defaults to 200 for everything and 500 for all errors. Use the right code.

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Bare return | `return Json(project)` | `return Json(ApiResponse { data: project.into() })` |
| Naked status code | `Err(StatusCode::NOT_FOUND)` | `Err(AppError::NotFound("project not found"))` with JSON body |
| Offset pagination | `LIMIT 20 OFFSET 40` | Keyset: `WHERE (created_at, id) < ($cursor_ts, $cursor_id)` |
| No pagination | `SELECT * FROM projects` on list endpoint | Always paginate. Default limit 25, max 100. |
| SELECT * in API | `query_as("SELECT * FROM ...")` | Name columns. Return DTO, not DB row. |
| Same struct for DB and API | `#[derive(sqlx::FromRow, Serialize)]` | Separate `ProjectRow` and `ProjectDto` |
| No input validation | Trust serde/Zod parse | Validate: string length, numeric range, cross-field rules |
| Inconsistent errors | `{ message }` vs `{ error }` vs `{ detail }` | One shape: `{ error: { code, message } }` |
| No versioning | `/projects` | `/v1/projects` |

## Cursor-Based Pagination

```rust
// You write (offset):
"SELECT ... ORDER BY created_at DESC LIMIT $1 OFFSET $2"

// Senior writes (keyset):
"SELECT ... WHERE (created_at, id) < ($1, $2) ORDER BY created_at DESC, id DESC LIMIT $3"
```

The pattern:
1. Fetch `limit + 1` rows
2. If you get `limit + 1`, there's a next page — pop the extra row
3. `next_cursor` = last returned row's ID
4. Client passes `cursor` on next request

Why not OFFSET: it's O(n) at the database level, and results shift when rows are inserted/deleted between pages.

## Error Type System

```rust
enum AppError {
    NotFound(String),
    Conflict(String),
    Validation(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::NotFound(msg) => (404, "not_found", msg),
            Self::Conflict(msg) => (409, "conflict", msg),
            Self::Validation(msg) => (422, "validation_error", msg),
            Self::Internal(err) => {
                error!(error = %err, "internal error");
                (500, "internal_error", "An internal error occurred".into())
            }
        };
        (StatusCode::from_u16(status).unwrap(),
         Json(json!({ "error": { "code": code, "message": message } })))
            .into_response()
    }
}
```

## DTO Separation

```rust
// Database model (internal):
#[derive(sqlx::FromRow)]
struct ProjectRow {
    id: Uuid,
    name: String,
    owner_id: Uuid,      // internal
    is_archived: bool,     // internal
    created_at: DateTime<Utc>,
}

// API response (external):
#[derive(Serialize)]
struct ProjectDto {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

impl From<ProjectRow> for ProjectDto {
    fn from(row: ProjectRow) -> Self {
        Self { id: row.id, name: row.name, created_at: row.created_at }
    }
}
```

Why: schema changes don't break the API. Internal fields don't leak. You control what clients see.

## Input Validation Beyond Deserialization

Serde/Zod/Pydantic parse types. You still need business rules:

```rust
impl CreateProjectRequest {
    fn validate(&self) -> Result<(), AppError> {
        if self.name.trim().is_empty() {
            return Err(AppError::Validation("name must not be blank".into()));
        }
        if self.name.len() > 200 {
            return Err(AppError::Validation("name must be 200 characters or fewer".into()));
        }
        Ok(())
    }
}
```

Always validate: string length limits, numeric ranges, required vs optional, cross-field consistency (e.g., `min_price <= max_price`).

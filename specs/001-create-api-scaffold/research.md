# Research: Rust API Scaffold with actix-web, Postgres, OpenAPI

## actix-web Best Practices
- Use latest stable actix-web for async REST APIs.
- Organize handlers, models, and error types in separate modules.
- Use actix-web extractors for path/query/body validation.
- Prefer `App::service()` for endpoint registration.
- Use middleware for logging, error handling, and CORS.

## PostgreSQL Integration
- Use `deadpool-postgres` for async connection pooling.
- Use `tokio-postgres` for direct DB access.
- Store DB config in `.env` and load with `dotenvy`.
- Use migrations (e.g., `refinery` or `sqlx` CLI) for schema management.

## OpenAPI Doc Generation
- Use `utoipa` and `utoipa-swagger-ui` for OpenAPI 3.0 docs.
- Annotate handlers and models with `#[derive(ToSchema)]` and endpoint macros.
- Serve docs at `/openapi.json` and `/swagger-ui/`.

## Error Handling & Response Patterns
- Use `thiserror` for custom error types.
- Implement `ResponseError` for actix-web error mapping.
- Standardize error responses to include `code`, `message`, `description`.
- For 422, include `errors` map for field-level validation errors.
- Use `validator` crate for struct validation.

## Pagination & Meta Extensibility
- Paginated endpoints should return `data` array and `meta` object.
- `meta` should include pagination info (page, per_page, total, etc.) and be extensible.
- For single resources, return `data` object and `meta`.
- Allow developers to add custom fields to `meta`.

## Project Structure (all code in project/)
- `project/`
  - `main.rs` (entrypoint)
  - `api/` (handlers/controllers)
  - `models/` (data models/entities)
  - `db/` (database pool/config)
  - `errors/` (error types/handlers)
  - `contracts/` (OpenAPI schemas)
  - `tests/` (integration/unit tests)

## Testing
- Use `cargo test` for unit/integration tests.
- Use actix-web test utilities for endpoint tests.
- Use a test Postgres DB for integration tests.

## Alternatives Considered
- Rocket (less async support, less mature OpenAPI integration)
- warp/tide (less batteries-included for OpenAPI/docs)
- sqlx (good, but deadpool-postgres is more direct for pool)

## Decision
- actix-web + deadpool-postgres + utoipa for OpenAPI, all code in project/, strict error/response patterns, extensible meta, and pagination support.

# Quickstart: Rust API Scaffold (actix-web, Postgres, OpenAPI)

## Prerequisites
- Rust (latest stable)
- PostgreSQL (running locally or via Docker)
- `cargo` (Rust package manager)

## Setup
1. Clone the repository and checkout the feature branch.
2. Copy `.env.example` to `.env` and set your Postgres connection string.
3. Run database migrations (if any).
4. Build and run the API:
   ```sh
   cd project
   cargo run
   ```
5. Access OpenAPI docs at `http://localhost:8080/swagger-ui/`.

## Adding Endpoints
- Add new handlers in `project/api/`.
- Register endpoints in `main.rs`.
- Define models in `project/models/`.
- Document endpoints and models with utoipa macros for OpenAPI.

## Error & Response Patterns
- All errors use the standard JSON format:
  ```json
  {
      "code": "domain.error_code",
      "message": "Some error message",
      "description": "Some error description"
  }
  ```
- Validation errors (422):
  ```json
  {
      "code": "validation.error",
      "message": "Some error message",
      "description": "Some error description",
      "errors": {
          "field_1": ["Field 1 error message 1"]
      }
  }
  ```
- Paginated/multiple resources:
  ```json
  {
      "data": [ ... ],
      "meta": { "page": 1, "per_page": 10, "total": 100 }
  }
  ```
- Single resource:
  ```json
  {
      "data": { ... },
      "meta": {}
  }
  ```
- Developers can add custom fields to `meta`.

## Testing
- Run all tests:
  ```sh
  cargo test
  ```
- Contract tests are in `specs/001-create-api-scaffold/contracts/`.

## Extending
- Add new models/entities in `project/models/`.
- Add new endpoints in `project/api/` and document with utoipa.
- Extend meta objects as needed for your use case.

---
For more details, see the OpenAPI spec in `contracts/openapi.yaml` and contract tests.

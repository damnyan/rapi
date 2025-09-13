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

## Example Endpoints

- **GET /example**: Returns a single example resource (see OpenAPI docs for schema).
- **POST /example**: Create a new example resource. Validates input fields and returns validation errors in standard format.

### Example: Create Example Resource
```sh
curl -X POST http://localhost:8080/example \
  -H 'Content-Type: application/json' \
  -d '{"id":1,"name":"Test Name"}'
```

#### Example Validation Error Response
```json
{
  "code": "validation.error",
  "message": "Validation failed",
  "description": "Validation error: ...",
  "errors": {
    "name": ["Name must not be empty"]
  }
}
```

## Adding Endpoints
- Add new handlers in `project/api/`.
- Register endpoints in `main.rs`.
- Define models in `project/models/`.
- Document endpoints and models with utoipa macros for OpenAPI.

## Error & Response Patterns

All endpoints use strict JSON response patterns for errors, validation, and resources. See OpenAPI docs for full schemas.

- **Error**:
  ```json
  {
    "code": "domain.error_code",
    "message": "Some error message",
    "description": "Some error description"
  }
  ```
- **Validation error (422)**:
  ```json
  {
    "code": "validation.error",
    "message": "Validation failed",
    "description": "Validation error: ...",
    "errors": {
      "field": ["Field error message"]
    }
  }
  ```
- **Paginated/multiple resources**:
  ```json
  {
    "data": [ ... ],
    "meta": { "page": 1, "per_page": 10, "total": 100 }
  }
  ```
- **Single resource**:
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
- Register new endpoints in `main.rs` and update OpenAPI schemas.
- Extend meta objects as needed for your use case.

---
For more details, see the OpenAPI spec in `contracts/openapi.yaml` and contract tests.

# Tasks: API Scaffold (Rust, actix-web, Postgres, OpenAPI)

**Input**: Design documents from `/specs/001-create-api-scaffold/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md for tech stack, structure, dependencies
2. Load data-model.md for entities
3. Load contracts/ for OpenAPI endpoints and contract tests
4. Load research.md and quickstart.md for setup and test scenarios
5. Generate tasks by dependency order and parallelizability
```

## Phase 3.1: Setup
- [x] T001 Create project/ structure and submodules per plan.md
- [x] T002 Initialize Rust project with actix-web, deadpool-postgres, utoipa, and other dependencies in Cargo.toml
- [x] T003 [P] Configure linting (rustfmt, clippy) and .env setup

## Phase 3.2: Tests First (TDD)
- [x] T004 [P] Contract test for error response schema in `specs/001-create-api-scaffold/contracts/example_contract_test.rs`
- [x] T005 [P] Contract test for validation error response schema in `specs/001-create-api-scaffold/contracts/example_contract_test.rs`
- [x] T006 [P] Contract test for paginated list response schema in `specs/001-create-api-scaffold/contracts/example_contract_test.rs`
- [x] T007 [P] Contract test for single resource response schema in `specs/001-create-api-scaffold/contracts/example_contract_test.rs`

## Phase 3.3: Core Implementation
- [x] T008 [P] Implement ErrorResponse, ValidationErrorResponse, PaginatedList, SingleResource, PaginationMeta, and Meta models in `project/models/`
- [x] T009 Implement GET /example endpoint in `project/api/` per OpenAPI spec
- [x] T010 Implement error handling and response mapping in `project/errors/`
- [x] T011 Implement OpenAPI doc generation and serve at `/openapi.json` and `/swagger-ui/` in `main.rs`

## Phase 3.4: Integration
- [x] T012 Set up Postgres connection pool in `project/db/` using deadpool-postgres
- [x] T013 Add structured logging and error context (tracing/log crate) in `main.rs` and middleware
- [x] T014 Add validation logic using validator crate in models and handlers

## Phase 3.5: Polish
- [x] T015 [P] Add unit tests for error and validation logic in `project/tests/`
- [x] T016 [P] Update quickstart.md and OpenAPI docs for new endpoints and models
- [x] T017 [P] Performance test: Ensure <200ms p95 latency under load

## Dependencies
- Setup (T001-T003) before all other tasks
- Contract tests (T004-T007) before implementation (T008-T011)
- Models (T008) before endpoints (T009)
- Error handling (T010) before OpenAPI doc (T011)
- Integration (T012-T014) after core implementation
- Polish (T015-T017) after all implementation

## Parallel Example
```
# Launch contract tests in parallel:
Task: "Contract test for error response schema in contracts/example_contract_test.rs"
Task: "Contract test for validation error response schema in contracts/example_contract_test.rs"
Task: "Contract test for paginated list response schema in contracts/example_contract_test.rs"
Task: "Contract test for single resource response schema in contracts/example_contract_test.rs"
```

## Notes
- [P] tasks = different files or independent logic
- All contract tests must fail before implementation
- Each task specifies exact file path
- Commit after each task
- Avoid: vague tasks, same file conflicts for [P] tasks

# Copilot Instructions for AI Coding Agents

## Project Overview

This repository is a Rust backend API scaffold using `actix-web`, `deadpool-postgres`, and `utoipa` for OpenAPI documentation. All code lives in the `project/` directory. The project is specification-driven: all features are planned, specified, and tracked in the `.specify/` and `specs/` directories using automated scripts and markdown templates.

## Architecture & Major Components

- **API Layer:** Handlers in `project/src/handlers/` (e.g., `get_example`) expose REST endpoints. Endpoints are registered in `main.rs`.
- **Models:** Data structures for API responses and errors are in `project/src/models/` (see `ErrorResponse`, `ValidationErrorResponse`, `PaginatedList`, `SingleResource`, `Meta`).
- **Error Handling:** Centralized in `project/src/errors/`, using `thiserror` and implementing `ResponseError` for actix-web.
- **Database:** (Planned) `project/db/` for Postgres connection pooling via `deadpool-postgres`.
- **OpenAPI Docs:** All endpoints and models are documented with `utoipa` macros. Docs are served at `/swagger-ui/` and `/openapi.json`.
- **Specs & Contracts:** All design, data model, and contract artifacts are in `specs/001-create-api-scaffold/`.

## Developer Workflows

- **Build & Run:**
  ```sh
  cd project
  cargo run
  ```
- **Test:**
  ```sh
  cargo test
  ```
- **Lint & Format:**
  ```sh
  cargo clippy -- -D warnings
  cargo fmt --all
  ```
- **Quickstart & Examples:** See `specs/001-create-api-scaffold/quickstart.md` for setup, endpoint patterns, and error/response examples.

## Specification-Driven Workflow

- **Plan a Feature:**
  - Run `.specify/scripts/bash/setup-plan.sh --json` to initialize a plan. Parse for `FEATURE_SPEC`, `IMPL_PLAN`, `SPECS_DIR`, and `BRANCH`.
  - Review `.specify/memory/constitution.md` for project principles.
  - Generate design docs in `$SPECS_DIR` using templates in `.specify/templates/`.
- **Create a Feature Spec:**
  - Use `.specify/scripts/bash/create-new-feature.sh --json "<desc>"` to create a feature branch and spec file.
  - Write the spec using `.specify/templates/spec-template.md`.
- **Generate Tasks:**
  - Use `.specify/scripts/bash/check-task-prerequisites.sh --json` to discover design docs.
  - Generate `tasks.md` using `.specify/templates/tasks-template.md`.

## Project Conventions & Patterns

- **All code in `project/`**; run all dev commands from there.
- **Absolute paths** from repo root for all scripts and file operations.
- **Artifacts** for each feature live in `specs/<feature>/`.
- **Error/response patterns** are strict—see `quickstart.md` and models for JSON structure.
- **Endpoints** must be registered in `main.rs` and documented with utoipa macros.
- **Meta objects** in responses are extensible for custom fields.
- **Contract tests** are in `specs/001-create-api-scaffold/contracts/` and must be written before implementation (TDD enforced).
- **Linting:** All warnings are errors (`Clippy.toml`).
- **Formatting:** Use `rustfmt.toml` (edition 2021).

## Key Files & Directories

- `project/src/main.rs` — Entrypoint, endpoint registration, OpenAPI docs
- `project/src/handlers/` — API handlers/controllers
- `project/src/models/` — Data models/entities
- `project/src/errors/` — Error types and mapping
- `project/db/` — (Planned) Database pool/config
- `specs/001-create-api-scaffold/` — All design, contract, and test artifacts
- `.specify/scripts/bash/` — Planning/specification automation scripts
- `.specify/templates/` — Markdown templates for all artifacts

## Commit Message Guidelines

- Use imperative mood: "Add feature", not "Added" or "Adds".
- Start with a concise summary (≤50 chars), capitalized, no period.
- Separate summary from body with a blank line.
- Explain _what_ and _why_ in the body, not _how_.
- Reference issues/specs if relevant (e.g., "See #42").

**Example:**

Add user authentication to API scaffold

Implements JWT-based login and registration endpoints.
Updates OpenAPI spec and adds tests. See 001-create-api-scaffold/spec.md.
